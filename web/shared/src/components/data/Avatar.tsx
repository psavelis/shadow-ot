import React from 'react'
import { cn } from '../../utils/cn'
import { User } from 'lucide-react'

interface AvatarProps {
  src?: string | null
  alt?: string
  fallback?: string
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  className?: string
  status?: 'online' | 'offline' | 'away' | 'busy'
}

const sizes = {
  xs: 'w-6 h-6 text-xs',
  sm: 'w-8 h-8 text-sm',
  md: 'w-10 h-10 text-base',
  lg: 'w-14 h-14 text-lg',
  xl: 'w-20 h-20 text-2xl',
}

const statusSizes = {
  xs: 'w-1.5 h-1.5 border',
  sm: 'w-2 h-2 border',
  md: 'w-2.5 h-2.5 border-2',
  lg: 'w-3 h-3 border-2',
  xl: 'w-4 h-4 border-2',
}

const statusColors = {
  online: 'bg-green-500',
  offline: 'bg-shadow-500',
  away: 'bg-yellow-500',
  busy: 'bg-red-500',
}

function getInitials(name: string): string {
  return name
    .split(' ')
    .map((part) => part[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)
}

export function Avatar({
  src,
  alt = 'Avatar',
  fallback,
  size = 'md',
  className,
  status,
}: AvatarProps) {
  const [error, setError] = React.useState(false)
  const showFallback = !src || error

  return (
    <div className={cn('relative inline-block', className)}>
      <div
        className={cn(
          'rounded-full bg-shadow-700 flex items-center justify-center overflow-hidden',
          sizes[size]
        )}
      >
        {showFallback ? (
          fallback ? (
            <span className="font-semibold text-shadow-300">
              {getInitials(fallback)}
            </span>
          ) : (
            <User className="w-1/2 h-1/2 text-shadow-500" />
          )
        ) : (
          <img
            src={src}
            alt={alt}
            onError={() => setError(true)}
            className="w-full h-full object-cover"
          />
        )}
      </div>
      {status && (
        <span
          className={cn(
            'absolute bottom-0 right-0 rounded-full border-shadow-900',
            statusColors[status],
            statusSizes[size],
            status === 'online' && 'animate-pulse'
          )}
        />
      )}
    </div>
  )
}

interface AvatarGroupProps {
  avatars: { src?: string; alt?: string; fallback?: string }[]
  max?: number
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  className?: string
}

export function AvatarGroup({ avatars, max = 4, size = 'md', className }: AvatarGroupProps) {
  const visibleAvatars = avatars.slice(0, max)
  const remainingCount = avatars.length - max

  const overlapSizes = {
    xs: '-ml-2',
    sm: '-ml-2',
    md: '-ml-3',
    lg: '-ml-4',
    xl: '-ml-5',
  }

  return (
    <div className={cn('flex items-center', className)}>
      {visibleAvatars.map((avatar, index) => (
        <div
          key={index}
          className={cn(
            'ring-2 ring-shadow-900 rounded-full',
            index > 0 && overlapSizes[size]
          )}
        >
          <Avatar {...avatar} size={size} />
        </div>
      ))}
      {remainingCount > 0 && (
        <div
          className={cn(
            'rounded-full bg-shadow-700 flex items-center justify-center ring-2 ring-shadow-900',
            sizes[size],
            overlapSizes[size]
          )}
        >
          <span className="text-shadow-300 font-medium">+{remainingCount}</span>
        </div>
      )}
    </div>
  )
}


