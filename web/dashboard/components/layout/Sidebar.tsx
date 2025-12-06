'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { 
  LayoutDashboard, 
  Users, 
  Wallet, 
  Settings, 
  Shield,
  Trophy,
  ShoppingCart,
  Home,
  Bell,
  HelpCircle,
  Swords,
  Crown,
  Sparkles,
  Activity,
  Gavel,
  Store,
  Users2,
  ChevronRight,
  Zap,
  Star
} from 'lucide-react'

const mainNav = [
  { name: 'Overview', href: '/dashboard', icon: LayoutDashboard },
  { name: 'Characters', href: '/dashboard/characters', icon: Users },
  { name: 'Live Activity', href: '/dashboard/live', icon: Activity },
  { name: 'Achievements', href: '/dashboard/achievements', icon: Trophy },
]

const tradingNav = [
  { name: 'Market', href: '/dashboard/market', icon: ShoppingCart },
  { name: 'Auction House', href: '/dashboard/auctions', icon: Gavel },
  { name: 'Houses', href: '/dashboard/houses', icon: Home },
]

const socialNav = [
  { name: 'My Guild', href: '/dashboard/guild', icon: Shield },
]

const financeNav = [
  { name: 'Wallet & NFTs', href: '/dashboard/wallet', icon: Wallet },
  { name: 'Premium & Shop', href: '/dashboard/premium', icon: Crown },
]

const supportNav = [
  { name: 'Support Center', href: '/dashboard/support', icon: HelpCircle },
  { name: 'Settings', href: '/dashboard/settings', icon: Settings },
]

const realms = [
  { name: 'Shadowlands', color: 'bg-amber-500', players: '1,247' },
  { name: 'Mythara', color: 'bg-emerald-500', players: '892' },
  { name: 'Aetheria', color: 'bg-purple-500', players: '645' },
  { name: 'Voidborne', color: 'bg-red-500', players: '423' },
  { name: 'Warbound', color: 'bg-orange-500', players: '312' },
]

export function Sidebar() {
  const pathname = usePathname()

  const NavSection = ({ title, items }: { title: string; items: typeof mainNav }) => (
    <div className="mb-4">
      <p className="px-4 text-[10px] font-semibold text-slate-500 uppercase tracking-wider mb-2">
        {title}
      </p>
      <div className="space-y-0.5">
        {items.map((item) => {
          const isActive = pathname === item.href || 
            (item.href !== '/dashboard' && pathname.startsWith(item.href))
          return (
            <Link
              key={item.name}
              href={item.href}
              className={`flex items-center gap-3 px-4 py-2 rounded-lg text-sm transition-all ${
                isActive 
                  ? 'bg-gradient-to-r from-amber-500/20 to-orange-500/10 text-amber-400 border-l-2 border-amber-500' 
                  : 'text-slate-400 hover:text-white hover:bg-slate-800/50'
              }`}
            >
              <item.icon className={`w-4 h-4 ${isActive ? 'text-amber-400' : ''}`} />
              <span>{item.name}</span>
            </Link>
          )
        })}
      </div>
    </div>
  )

  return (
    <aside className="w-64 bg-slate-900/80 border-r border-slate-800 flex flex-col backdrop-blur-sm">
      {/* Logo */}
      <div className="h-16 flex items-center px-5 border-b border-slate-800">
        <Link href="/" className="flex items-center gap-3 group">
          <div className="w-10 h-10 bg-gradient-to-br from-amber-500 to-orange-600 rounded-xl flex items-center justify-center shadow-lg shadow-amber-500/20 group-hover:shadow-amber-500/40 transition-shadow">
            <span className="font-bold text-lg text-white">S</span>
          </div>
          <div>
            <span className="font-bold text-lg text-white">Shadow</span>
            <span className="font-bold text-lg text-amber-500">OT</span>
          </div>
        </Link>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-4 px-3 space-y-1">
        <NavSection title="Main" items={mainNav} />
        <NavSection title="Trading & Economy" items={tradingNav} />
        <NavSection title="Social" items={socialNav} />
        <NavSection title="Finance" items={financeNav} />
        
        {/* Realms Quick View */}
        <div className="mb-4 pt-2">
          <p className="px-4 text-[10px] font-semibold text-slate-500 uppercase tracking-wider mb-2">
            Realms Online
          </p>
          <div className="space-y-1 px-2">
            {realms.map((realm) => (
              <div
                key={realm.name}
                className="flex items-center justify-between px-2 py-1.5 rounded-lg hover:bg-slate-800/50 transition cursor-pointer group"
              >
                <div className="flex items-center gap-2">
                  <div className={`w-2 h-2 ${realm.color} rounded-full animate-pulse`} />
                  <span className="text-xs text-slate-400 group-hover:text-white transition">{realm.name}</span>
                </div>
                <span className="text-[10px] text-slate-500">{realm.players}</span>
              </div>
            ))}
          </div>
        </div>

        <div className="border-t border-slate-800 pt-4">
          <NavSection title="Account" items={supportNav} />
        </div>
      </nav>

      {/* Premium Status */}
      <div className="p-3 border-t border-slate-800">
        <Link
          href="/dashboard/premium"
          className="block p-3 bg-gradient-to-r from-amber-500/10 via-orange-500/10 to-red-500/10 border border-amber-500/30 rounded-xl hover:border-amber-500/50 transition group"
        >
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 bg-gradient-to-br from-amber-500 to-orange-600 rounded-lg flex items-center justify-center">
              <Crown className="w-4 h-4 text-white" />
            </div>
            <div className="flex-1">
              <p className="text-sm font-medium text-white group-hover:text-amber-400 transition">Premium Active</p>
              <p className="text-[10px] text-slate-500">89 days remaining</p>
            </div>
            <ChevronRight className="w-4 h-4 text-slate-600 group-hover:text-amber-400 transition" />
          </div>
        </Link>
      </div>
    </aside>
  )
}
