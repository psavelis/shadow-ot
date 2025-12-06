'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { 
  LayoutDashboard, 
  Server, 
  Users, 
  Globe, 
  Settings, 
  Shield,
  FileText,
  Activity,
  AlertTriangle,
  MessageSquare,
  Database,
  Terminal
} from 'lucide-react'

const navigation = [
  { name: 'Dashboard', href: '/', icon: LayoutDashboard },
  { name: 'Server Control', href: '/servers', icon: Server },
  { name: 'Players', href: '/players', icon: Users },
  { name: 'Realms', href: '/realms', icon: Globe },
  { name: 'Moderation', href: '/moderation', icon: Shield },
  { name: 'Analytics', href: '/analytics', icon: Activity },
  { name: 'Logs', href: '/logs', icon: FileText },
  { name: 'Reports', href: '/reports', icon: AlertTriangle },
  { name: 'Announcements', href: '/announcements', icon: MessageSquare },
  { name: 'Database', href: '/database', icon: Database },
  { name: 'Console', href: '/console', icon: Terminal },
  { name: 'Settings', href: '/settings', icon: Settings },
]

export function AdminSidebar() {
  const pathname = usePathname()

  return (
    <aside className="w-64 bg-shadow-900/50 border-r border-shadow-800 flex flex-col">
      <div className="h-16 flex items-center px-6 border-b border-shadow-800">
        <Link href="/" className="flex items-center space-x-3">
          <div className="w-9 h-9 bg-gradient-to-br from-red-500 to-red-700 rounded-lg flex items-center justify-center">
            <Shield className="w-5 h-5 text-white" />
          </div>
          <span className="font-display font-bold text-lg">Admin Panel</span>
        </Link>
      </div>

      <nav className="flex-1 overflow-y-auto py-6 px-4 space-y-1">
        {navigation.map((item) => {
          const isActive = pathname === item.href
          return (
            <Link key={item.name} href={item.href} className={`sidebar-link ${isActive ? 'active' : ''}`}>
              <item.icon className="w-5 h-5" />
              <span>{item.name}</span>
            </Link>
          )
        })}
      </nav>

      <div className="p-4 border-t border-shadow-800">
        <div className="p-3 bg-green-500/10 border border-green-500/30 rounded-lg">
          <div className="flex items-center space-x-2 mb-1">
            <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
            <span className="text-green-400 text-sm font-medium">All Systems Online</span>
          </div>
          <p className="text-shadow-400 text-xs">Last check: 30s ago</p>
        </div>
      </div>
    </aside>
  )
}

