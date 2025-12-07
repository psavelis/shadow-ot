import React from 'react'
import { cn } from '../../utils/cn'
import { RealmId } from '../../types'

interface RealmBadgeProps {
  realm: RealmId
  size?: 'sm' | 'md' | 'lg'
  showIcon?: boolean
  className?: string
}

const realmConfig: Record<RealmId, { name: string; icon: string; color: string; bg: string }> = {
  shadowveil: { name: 'Shadowveil', icon: '🌑', color: 'text-red-400', bg: 'bg-red-500/20' },
  aetheria: { name: 'Aetheria', icon: '✨', color: 'text-blue-400', bg: 'bg-blue-500/20' },
  warbound: { name: 'Warbound', icon: '⚔️', color: 'text-orange-400', bg: 'bg-orange-500/20' },
  mythara: { name: 'Mythara', icon: '🏰', color: 'text-green-400', bg: 'bg-green-500/20' },
  voidborne: { name: 'Voidborne', icon: '🌀', color: 'text-purple-400', bg: 'bg-purple-500/20' },
  grimhollow: { name: 'Grimhollow', icon: '💀', color: 'text-slate-400', bg: 'bg-slate-500/20' },
}

const sizes = {
  sm: 'px-1.5 py-0.5 text-xs',
  md: 'px-2.5 py-1 text-xs',
  lg: 'px-3 py-1.5 text-sm',
}

export function RealmBadge({ realm, size = 'md', showIcon = true, className }: RealmBadgeProps) {
  const config = realmConfig[realm]
  if (!config) return null

  return (
    <span className={cn('inline-flex items-center gap-1 rounded-full font-medium', config.bg, config.color, sizes[size], className)}>
      {showIcon && <span>{config.icon}</span>}
      <span>{config.name}</span>
    </span>
  )
}

export function getRealmConfig(realm: RealmId) {
  return realmConfig[realm]
}


