'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import { Bell, Search, ChevronDown, LogOut, Settings, User, Wallet, Activity, Command, Globe, Zap } from 'lucide-react'
import * as DropdownMenu from '@radix-ui/react-dropdown-menu'
import * as Dialog from '@radix-ui/react-dialog'
import { ConnectButton } from '@rainbow-me/rainbowkit'

const notifications = [
  { id: 1, title: 'Level Up!', message: 'ShadowKnight reached level 342', time: '5m ago', unread: true, type: 'levelup' },
  { id: 2, title: 'Trade Completed', message: 'Demon Helmet sold for 500k', time: '1h ago', unread: true, type: 'trade' },
  { id: 3, title: 'Guild Message', message: 'Double XP event this weekend!', time: '3h ago', unread: false, type: 'guild' },
]

const searchResults = [
  { type: 'character', name: 'ShadowKnight', desc: 'Level 342 Elite Knight' },
  { type: 'item', name: 'Demon Armor', desc: 'Equipped by ShadowKnight' },
  { type: 'page', name: 'Market', desc: 'Buy and sell items' },
]

export function TopBar() {
  const [showSearch, setShowSearch] = useState(false)
  const [searchQuery, setSearchQuery] = useState('')
  const [serverStatus, setServerStatus] = useState<'online' | 'degraded' | 'offline'>('online')
  const [playersOnline, setPlayersOnline] = useState(3519)
  
  const unreadCount = notifications.filter(n => n.unread).length

  // Keyboard shortcut for search
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault()
        setShowSearch(true)
      }
      if (e.key === 'Escape') {
        setShowSearch(false)
      }
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [])

  return (
    <>
      <header className="h-14 bg-slate-900/80 border-b border-slate-800 flex items-center justify-between px-6 backdrop-blur-sm">
        {/* Left: Search Trigger */}
        <button
          onClick={() => setShowSearch(true)}
          className="flex items-center gap-3 px-4 py-2 bg-slate-800/50 border border-slate-700/50 rounded-lg hover:border-slate-600 transition group w-72"
        >
          <Search className="w-4 h-4 text-slate-500" />
          <span className="text-slate-500 text-sm flex-1 text-left">Search...</span>
          <kbd className="hidden sm:inline-flex items-center gap-1 px-2 py-0.5 bg-slate-700/50 text-slate-400 text-xs rounded">
            <Command className="w-3 h-3" />K
          </kbd>
        </button>

        {/* Center: Server Status */}
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2 px-3 py-1.5 bg-slate-800/50 border border-slate-700/50 rounded-lg">
            <div className={`w-2 h-2 rounded-full ${
              serverStatus === 'online' ? 'bg-emerald-400 animate-pulse' :
              serverStatus === 'degraded' ? 'bg-amber-400 animate-pulse' :
              'bg-red-400'
            }`} />
            <span className="text-slate-400 text-xs">
              {serverStatus === 'online' ? 'All Systems Operational' :
               serverStatus === 'degraded' ? 'Degraded Performance' :
               'Systems Offline'}
            </span>
          </div>
          <div className="flex items-center gap-2 px-3 py-1.5 bg-slate-800/50 border border-slate-700/50 rounded-lg">
            <Activity className="w-3 h-3 text-emerald-400" />
            <span className="text-white text-xs font-medium">{playersOnline.toLocaleString()}</span>
            <span className="text-slate-500 text-xs">online</span>
          </div>
        </div>

        {/* Right: Actions */}
        <div className="flex items-center gap-3">
          {/* Wallet Connect */}
          <ConnectButton.Custom>
            {({ account, chain, openConnectModal, openAccountModal, mounted }) => {
              const connected = mounted && account && chain
              return (
                <button
                  onClick={connected ? openAccountModal : openConnectModal}
                  className="flex items-center gap-2 px-3 py-2 bg-slate-800/50 hover:bg-slate-700 rounded-lg border border-slate-700/50 transition text-sm"
                >
                  <Wallet className="w-4 h-4 text-purple-400" />
                  <span className="text-slate-300 font-medium">
                    {connected ? account.displayName : 'Connect'}
                  </span>
                </button>
              )
            }}
          </ConnectButton.Custom>

          {/* Notifications */}
          <DropdownMenu.Root>
            <DropdownMenu.Trigger asChild>
              <button className="relative p-2 text-slate-400 hover:text-white hover:bg-slate-800/50 rounded-lg transition">
                <Bell className="w-5 h-5" />
                {unreadCount > 0 && (
                  <span className="absolute top-1 right-1 w-4 h-4 bg-amber-500 rounded-full text-[10px] font-bold text-white flex items-center justify-center">
                    {unreadCount}
                  </span>
                )}
              </button>
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
              <DropdownMenu.Content
                className="w-80 bg-slate-900 border border-slate-700 rounded-xl shadow-2xl overflow-hidden z-50"
                sideOffset={8}
                align="end"
              >
                <div className="p-4 border-b border-slate-800">
                  <div className="flex items-center justify-between">
                    <h3 className="font-semibold text-white">Notifications</h3>
                    <button className="text-xs text-amber-500 hover:text-amber-400">
                      Mark all read
                    </button>
                  </div>
                </div>
                <div className="max-h-80 overflow-y-auto">
                  {notifications.map((notification) => (
                    <DropdownMenu.Item
                      key={notification.id}
                      className="p-4 hover:bg-slate-800/50 cursor-pointer border-b border-slate-800/50 last:border-0 focus:outline-none"
                    >
                      <div className="flex items-start gap-3">
                        {notification.unread && (
                          <div className="w-2 h-2 bg-amber-500 rounded-full mt-2 flex-shrink-0" />
                        )}
                        <div className={notification.unread ? '' : 'ml-5'}>
                          <p className="text-sm font-medium text-white">{notification.title}</p>
                          <p className="text-xs text-slate-400 mt-0.5">{notification.message}</p>
                          <p className="text-xs text-slate-500 mt-1">{notification.time}</p>
                        </div>
                      </div>
                    </DropdownMenu.Item>
                  ))}
                </div>
                <div className="p-3 border-t border-slate-800">
                  <Link
                    href="/dashboard/notifications"
                    className="block text-center text-sm text-amber-500 hover:text-amber-400"
                  >
                    View all notifications
                  </Link>
                </div>
              </DropdownMenu.Content>
            </DropdownMenu.Portal>
          </DropdownMenu.Root>

          {/* User Menu */}
          <DropdownMenu.Root>
            <DropdownMenu.Trigger asChild>
              <button className="flex items-center gap-3 p-1.5 hover:bg-slate-800/50 rounded-lg transition">
                <div className="w-8 h-8 bg-gradient-to-br from-amber-500 to-orange-600 rounded-lg flex items-center justify-center shadow-lg shadow-amber-500/20">
                  <span className="text-white text-sm font-bold">S</span>
                </div>
                <div className="text-left hidden sm:block">
                  <p className="text-sm font-medium text-white">ShadowMaster</p>
                  <p className="text-[10px] text-amber-400 flex items-center gap-1">
                    <Zap className="w-3 h-3" /> Premium
                  </p>
                </div>
                <ChevronDown className="w-4 h-4 text-slate-500" />
              </button>
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
              <DropdownMenu.Content
                className="w-56 bg-slate-900 border border-slate-700 rounded-xl shadow-2xl overflow-hidden z-50"
                sideOffset={8}
                align="end"
              >
                <div className="p-3 border-b border-slate-800">
                  <p className="text-sm font-medium text-white">ShadowMaster</p>
                  <p className="text-xs text-slate-500">shadowmaster@email.com</p>
                </div>
                <div className="p-2">
                  <DropdownMenu.Item asChild>
                    <Link
                      href="/dashboard/settings"
                      className="flex items-center gap-3 px-3 py-2 text-slate-300 hover:text-white hover:bg-slate-800 rounded-lg transition focus:outline-none"
                    >
                      <User className="w-4 h-4" />
                      <span className="text-sm">Profile</span>
                    </Link>
                  </DropdownMenu.Item>
                  <DropdownMenu.Item asChild>
                    <Link
                      href="/dashboard/settings"
                      className="flex items-center gap-3 px-3 py-2 text-slate-300 hover:text-white hover:bg-slate-800 rounded-lg transition focus:outline-none"
                    >
                      <Settings className="w-4 h-4" />
                      <span className="text-sm">Settings</span>
                    </Link>
                  </DropdownMenu.Item>
                </div>
                <div className="p-2 border-t border-slate-800">
                  <DropdownMenu.Item asChild>
                    <button className="flex items-center gap-3 px-3 py-2 text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded-lg transition w-full focus:outline-none">
                      <LogOut className="w-4 h-4" />
                      <span className="text-sm">Sign out</span>
                    </button>
                  </DropdownMenu.Item>
                </div>
              </DropdownMenu.Content>
            </DropdownMenu.Portal>
          </DropdownMenu.Root>
        </div>
      </header>

      {/* Command Palette / Search Dialog */}
      <Dialog.Root open={showSearch} onOpenChange={setShowSearch}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-[20%] left-1/2 -translate-x-1/2 w-full max-w-xl bg-slate-900 border border-slate-700 rounded-2xl shadow-2xl overflow-hidden z-50">
            <div className="flex items-center gap-3 px-4 py-3 border-b border-slate-800">
              <Search className="w-5 h-5 text-slate-500" />
              <input
                type="text"
                value={searchQuery}
                onChange={e => setSearchQuery(e.target.value)}
                placeholder="Search characters, items, pages..."
                className="flex-1 bg-transparent text-white placeholder-slate-500 focus:outline-none"
                autoFocus
              />
              <kbd className="px-2 py-1 bg-slate-800 text-slate-400 text-xs rounded">ESC</kbd>
            </div>
            
            {searchQuery ? (
              <div className="max-h-80 overflow-y-auto">
                {searchResults.filter(r => 
                  r.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                  r.desc.toLowerCase().includes(searchQuery.toLowerCase())
                ).map((result, idx) => (
                  <button
                    key={idx}
                    onClick={() => setShowSearch(false)}
                    className="w-full flex items-center gap-4 px-4 py-3 hover:bg-slate-800/50 transition text-left"
                  >
                    <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${
                      result.type === 'character' ? 'bg-amber-500/20' :
                      result.type === 'item' ? 'bg-purple-500/20' :
                      'bg-blue-500/20'
                    }`}>
                      {result.type === 'character' ? <User className="w-5 h-5 text-amber-400" /> :
                       result.type === 'item' ? <Wallet className="w-5 h-5 text-purple-400" /> :
                       <Globe className="w-5 h-5 text-blue-400" />}
                    </div>
                    <div>
                      <p className="text-white font-medium">{result.name}</p>
                      <p className="text-slate-500 text-sm">{result.desc}</p>
                    </div>
                    <span className="ml-auto text-slate-600 text-xs capitalize">{result.type}</span>
                  </button>
                ))}
              </div>
            ) : (
              <div className="p-4">
                <p className="text-slate-500 text-sm mb-3">Quick Actions</p>
                <div className="grid grid-cols-2 gap-2">
                  {[
                    { label: 'Create Character', icon: User, href: '/dashboard/characters/create' },
                    { label: 'View Market', icon: Globe, href: '/dashboard/market' },
                    { label: 'Mint NFT', icon: Wallet, href: '/dashboard/wallet' },
                    { label: 'Settings', icon: Settings, href: '/dashboard/settings' },
                  ].map((action, idx) => (
                    <Link
                      key={idx}
                      href={action.href}
                      onClick={() => setShowSearch(false)}
                      className="flex items-center gap-3 p-3 bg-slate-800/50 rounded-lg hover:bg-slate-800 transition"
                    >
                      <action.icon className="w-4 h-4 text-slate-400" />
                      <span className="text-slate-300 text-sm">{action.label}</span>
                    </Link>
                  ))}
                </div>
              </div>
            )}
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </>
  )
}
