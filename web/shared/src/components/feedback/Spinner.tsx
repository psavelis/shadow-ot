import React from 'react'
import { cn } from '../../utils/cn'
import { Loader2 } from 'lucide-react'

interface SpinnerProps {
  size?: 'sm' | 'md' | 'lg' | 'xl'
  className?: string
  label?: string
}

const sizes = {
  sm: 'w-4 h-4',
  md: 'w-6 h-6',
  lg: 'w-8 h-8',
  xl: 'w-12 h-12',
}

export function Spinner({ size = 'md', className, label }: SpinnerProps) {
  return (
    <div className={cn('inline-flex items-center gap-2', className)}>
      <Loader2 className={cn('animate-spin text-accent-500', sizes[size])} />
      {label && <span className="text-shadow-400 text-sm">{label}</span>}
    </div>
  )
}

// Full page loading spinner
interface LoadingScreenProps {
  message?: string
}

export function LoadingScreen({ message = 'Loading...' }: LoadingScreenProps) {
  return (
    <div className="min-h-screen flex items-center justify-center bg-shadow-950">
      <div className="text-center">
        <Spinner size="xl" />
        <p className="mt-4 text-shadow-400">{message}</p>
      </div>
    </div>
  )
}

// Inline loading state
interface LoadingOverlayProps {
  loading: boolean
  children: React.ReactNode
  blur?: boolean
  className?: string
}

export function LoadingOverlay({
  loading,
  children,
  blur = true,
  className,
}: LoadingOverlayProps) {
  return (
    <div className={cn('relative', className)}>
      {children}
      {loading && (
        <div
          className={cn(
            'absolute inset-0 flex items-center justify-center bg-shadow-900/50',
            blur && 'backdrop-blur-sm'
          )}
        >
          <Spinner size="lg" />
        </div>
      )}
    </div>
  )
}

