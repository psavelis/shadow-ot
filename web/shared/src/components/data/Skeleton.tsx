import React from 'react'
import { cn } from '../../utils/cn'

interface SkeletonProps {
  className?: string
  variant?: 'text' | 'circular' | 'rectangular'
  animation?: 'pulse' | 'wave' | 'none'
}

export function Skeleton({
  className,
  variant = 'text',
  animation = 'pulse',
}: SkeletonProps) {
  const variants = {
    text: 'h-4 rounded',
    circular: 'rounded-full',
    rectangular: 'rounded-lg',
  }

  const animations = {
    pulse: 'animate-pulse',
    wave: 'animate-shimmer',
    none: '',
  }

  return (
    <div
      className={cn(
        'bg-shadow-700',
        variants[variant],
        animations[animation],
        className
      )}
    />
  )
}

// Skeleton text block
interface SkeletonTextProps {
  lines?: number
  className?: string
}

export function SkeletonText({ lines = 3, className }: SkeletonTextProps) {
  return (
    <div className={cn('space-y-2', className)}>
      {Array.from({ length: lines }).map((_, i) => (
        <Skeleton
          key={i}
          className={cn('h-4', i === lines - 1 && 'w-3/4')}
        />
      ))}
    </div>
  )
}

// Skeleton card
interface SkeletonCardProps {
  className?: string
  showImage?: boolean
  showTitle?: boolean
  lines?: number
}

export function SkeletonCard({
  className,
  showImage = true,
  showTitle = true,
  lines = 2,
}: SkeletonCardProps) {
  return (
    <div
      className={cn(
        'bg-shadow-900/50 border border-shadow-700 rounded-xl p-4',
        className
      )}
    >
      {showImage && <Skeleton className="h-40 w-full mb-4" variant="rectangular" />}
      {showTitle && <Skeleton className="h-6 w-3/4 mb-3" />}
      <SkeletonText lines={lines} />
    </div>
  )
}

// Skeleton table
interface SkeletonTableProps {
  rows?: number
  columns?: number
  className?: string
}

export function SkeletonTable({
  rows = 5,
  columns = 4,
  className,
}: SkeletonTableProps) {
  return (
    <div className={cn('overflow-x-auto', className)}>
      <table className="w-full">
        <thead>
          <tr className="border-b border-shadow-700">
            {Array.from({ length: columns }).map((_, i) => (
              <th key={i} className="py-3 px-4">
                <Skeleton className="h-4 w-20" />
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {Array.from({ length: rows }).map((_, rowIndex) => (
            <tr key={rowIndex} className="border-b border-shadow-800">
              {Array.from({ length: columns }).map((_, colIndex) => (
                <td key={colIndex} className="py-3 px-4">
                  <Skeleton className="h-4" />
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

// Skeleton avatar
export function SkeletonAvatar({ size = 'md' }: { size?: 'sm' | 'md' | 'lg' }) {
  const sizes = {
    sm: 'w-8 h-8',
    md: 'w-10 h-10',
    lg: 'w-14 h-14',
  }

  return <Skeleton variant="circular" className={sizes[size]} />
}

// Skeleton stat card
export function SkeletonStatCard() {
  return (
    <div className="bg-shadow-900/50 border border-shadow-700 rounded-xl p-5">
      <div className="flex items-center justify-between mb-3">
        <Skeleton variant="rectangular" className="w-10 h-10" />
        <Skeleton className="w-12 h-4" />
      </div>
      <Skeleton className="h-8 w-20 mb-2" />
      <Skeleton className="h-4 w-16" />
    </div>
  )
}


