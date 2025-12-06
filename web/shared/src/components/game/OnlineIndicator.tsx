import React from 'react'
import { cn } from '../../utils/cn'

interface OnlineIndicatorProps {
  online: boolean
  size?: 'sm' | 'md' | 'lg'
  showLabel?: boolean
  className?: string
  pulse?: boolean
}

const sizes = {
  sm: 'w-2 h-2',
  md: 'w-2.5 h-2.5',
  lg: 'w-3 h-3',
}

export function OnlineIndicator({ online, size = 'md', showLabel = false, className, pulse = true }: OnlineIndicatorProps) {
  if (showLabel) {
    return (
      <span className={cn('inline-flex items-center gap-1.5', className)}>
        <span className={cn('rounded-full', sizes[size], online ? 'bg-green-500' : 'bg-shadow-500', online && pulse && 'animate-pulse')} />
        <span className={cn('text-sm', online ? 'text-green-400' : 'text-shadow-500')}>{online ? 'Online' : 'Offline'}</span>
      </span>
    )
  }

  return <span className={cn('inline-block rounded-full', sizes[size], online ? 'bg-green-500' : 'bg-shadow-500', online && pulse && 'animate-pulse', className)} />
}

