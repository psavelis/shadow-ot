import React from 'react'
import { cn } from '../../utils/cn'

interface BadgeProps {
  children: React.ReactNode
  variant?: 'default' | 'success' | 'warning' | 'danger' | 'info' | 'accent' | 'outline'
  size?: 'sm' | 'md' | 'lg'
  className?: string
  dot?: boolean
  pulse?: boolean
}

const variants = {
  default: 'bg-shadow-700 text-shadow-300',
  success: 'bg-green-500/20 text-green-400',
  warning: 'bg-yellow-500/20 text-yellow-400',
  danger: 'bg-red-500/20 text-red-400',
  info: 'bg-blue-500/20 text-blue-400',
  accent: 'bg-accent-500/20 text-accent-400',
  outline: 'bg-transparent border border-shadow-600 text-shadow-300',
}

const sizes = {
  sm: 'px-1.5 py-0.5 text-xs',
  md: 'px-2.5 py-1 text-xs',
  lg: 'px-3 py-1.5 text-sm',
}

const dotColors = {
  default: 'bg-shadow-400',
  success: 'bg-green-400',
  warning: 'bg-yellow-400',
  danger: 'bg-red-400',
  info: 'bg-blue-400',
  accent: 'bg-accent-400',
  outline: 'bg-shadow-400',
}

export function Badge({
  children,
  variant = 'default',
  size = 'md',
  className,
  dot = false,
  pulse = false,
}: BadgeProps) {
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 font-medium rounded-full',
        variants[variant],
        sizes[size],
        className
      )}
    >
      {dot && (
        <span
          className={cn(
            'w-1.5 h-1.5 rounded-full',
            dotColors[variant],
            pulse && 'animate-pulse'
          )}
        />
      )}
      {children}
    </span>
  )
}

// Realm-specific badge
interface RealmBadgeProps {
  realm: string
  size?: 'sm' | 'md' | 'lg'
  className?: string
}

const realmColors: Record<string, string> = {
  shadowveil: 'bg-red-500/20 text-red-400',
  aetheria: 'bg-blue-500/20 text-blue-400',
  warbound: 'bg-orange-500/20 text-orange-400',
  mythara: 'bg-green-500/20 text-green-400',
  voidborne: 'bg-purple-500/20 text-purple-400',
  grimhollow: 'bg-slate-500/20 text-slate-400',
}

export function RealmBadge({ realm, size = 'md', className }: RealmBadgeProps) {
  const realmId = realm.toLowerCase()
  const colorClass = realmColors[realmId] || variants.default

  return (
    <span
      className={cn(
        'inline-flex items-center font-medium rounded-full capitalize',
        colorClass,
        sizes[size],
        className
      )}
    >
      {realm}
    </span>
  )
}

// Status badge
interface StatusBadgeProps {
  status: 'online' | 'offline' | 'away' | 'busy' | 'maintenance'
  showLabel?: boolean
  size?: 'sm' | 'md' | 'lg'
  className?: string
}

const statusConfig = {
  online: { color: 'bg-green-500', label: 'Online' },
  offline: { color: 'bg-shadow-500', label: 'Offline' },
  away: { color: 'bg-yellow-500', label: 'Away' },
  busy: { color: 'bg-red-500', label: 'Busy' },
  maintenance: { color: 'bg-orange-500', label: 'Maintenance' },
}

export function StatusBadge({
  status,
  showLabel = false,
  size = 'md',
  className,
}: StatusBadgeProps) {
  const config = statusConfig[status]

  if (!showLabel) {
    return (
      <span
        className={cn(
          'inline-block rounded-full',
          config.color,
          size === 'sm' && 'w-2 h-2',
          size === 'md' && 'w-2.5 h-2.5',
          size === 'lg' && 'w-3 h-3',
          status === 'online' && 'animate-pulse',
          className
        )}
      />
    )
  }

  return (
    <Badge
      variant={status === 'online' ? 'success' : status === 'offline' ? 'default' : 'warning'}
      size={size}
      dot
      pulse={status === 'online'}
      className={className}
    >
      {config.label}
    </Badge>
  )
}


