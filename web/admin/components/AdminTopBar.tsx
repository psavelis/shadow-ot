'use client'

import { Bell, Search, ChevronDown, LogOut, Settings, User } from 'lucide-react'
import * as DropdownMenu from '@radix-ui/react-dropdown-menu'

export function AdminTopBar() {
  return (
    <header className="h-16 bg-shadow-900/50 border-b border-shadow-800 flex items-center justify-between px-6">
      <div className="relative w-96">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
        <input
          type="text"
          placeholder="Search players, logs, settings..."
          className="w-full pl-10 pr-4 py-2 bg-shadow-800 border border-shadow-700 rounded-lg text-white placeholder-shadow-500 focus:outline-none focus:border-accent-500"
        />
      </div>

      <div className="flex items-center space-x-4">
        <button className="relative p-2 text-shadow-400 hover:text-white">
          <Bell className="w-5 h-5" />
          <span className="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full" />
        </button>

        <DropdownMenu.Root>
          <DropdownMenu.Trigger asChild>
            <button className="flex items-center space-x-3 p-2 hover:bg-shadow-800 rounded-lg">
              <div className="w-8 h-8 bg-gradient-to-br from-red-500 to-orange-500 rounded-full flex items-center justify-center">
                <span className="text-white text-sm font-bold">A</span>
              </div>
              <div className="text-left">
                <p className="text-sm font-medium text-white">Admin</p>
                <p className="text-xs text-shadow-400">Super Admin</p>
              </div>
              <ChevronDown className="w-4 h-4 text-shadow-400" />
            </button>
          </DropdownMenu.Trigger>
          <DropdownMenu.Portal>
            <DropdownMenu.Content className="w-56 bg-shadow-900 border border-shadow-700 rounded-xl shadow-xl overflow-hidden z-50" sideOffset={8} align="end">
              <div className="p-2">
                <DropdownMenu.Item className="flex items-center space-x-3 px-3 py-2 text-shadow-300 hover:text-white hover:bg-shadow-800 rounded-lg cursor-pointer focus:outline-none">
                  <User className="w-4 h-4" /><span className="text-sm">Profile</span>
                </DropdownMenu.Item>
                <DropdownMenu.Item className="flex items-center space-x-3 px-3 py-2 text-shadow-300 hover:text-white hover:bg-shadow-800 rounded-lg cursor-pointer focus:outline-none">
                  <Settings className="w-4 h-4" /><span className="text-sm">Settings</span>
                </DropdownMenu.Item>
              </div>
              <div className="p-2 border-t border-shadow-700">
                <DropdownMenu.Item className="flex items-center space-x-3 px-3 py-2 text-red-400 hover:text-red-300 hover:bg-shadow-800 rounded-lg cursor-pointer focus:outline-none">
                  <LogOut className="w-4 h-4" /><span className="text-sm">Sign out</span>
                </DropdownMenu.Item>
              </div>
            </DropdownMenu.Content>
          </DropdownMenu.Portal>
        </DropdownMenu.Root>
      </div>
    </header>
  )
}

