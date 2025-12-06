import React from 'react'
import { cn } from '../../utils/cn'

interface ProgressProps {
  value: number
  max?: number
  size?: 'sm' | 'md' | 'lg'
  variant?: 'default' | 'success' | 'warning' | 'danger' | 'gradient'
  showLabel?: boolean
  label?: string
  className?: string
  animated?: boolean
}

const sizes = {
  sm: 'h-1.5',
  md: 'h-2.5',
  lg: 'h-4',
}

const barVariants = {
  default: 'bg-accent-500',
  success: 'bg-green-500',
  warning: 'bg-yellow-500',
  danger: 'bg-red-500',
  gradient: 'bg-gradient-to-r from-accent-500 to-purple-500',
}

export function Progress({
  value,
  max = 100,
  size = 'md',
  variant = 'default',
  showLabel = false,
  label,
  className,
  animated = false,
}: ProgressProps) {
  const percent = Math.min(Math.max((value / max) * 100, 0), 100)

  return (
    <div className={cn('w-full', className)}>
      {(showLabel || label) && (
        <div className="flex items-center justify-between mb-1.5 text-sm">
          {label && <span className="text-shadow-300">{label}</span>}
          {showLabel && (
            <span className="text-shadow-400">{percent.toFixed(0)}%</span>
          )}
        </div>
      )}
      <div
        className={cn(
          'w-full bg-shadow-700 rounded-full overflow-hidden',
          sizes[size]
        )}
        role="progressbar"
        aria-valuenow={value}
        aria-valuemin={0}
        aria-valuemax={max}
      >
        <div
          className={cn(
            'h-full rounded-full transition-all duration-500 ease-out',
            barVariants[variant],
            animated && 'animate-pulse'
          )}
          style={{ width: `${percent}%` }}
        />
      </div>
    </div>
  )
}

// Circular progress
interface CircularProgressProps {
  value: number
  max?: number
  size?: number
  strokeWidth?: number
  variant?: 'default' | 'success' | 'warning' | 'danger'
  showValue?: boolean
  className?: string
}

const circleVariants = {
  default: 'stroke-accent-500',
  success: 'stroke-green-500',
  warning: 'stroke-yellow-500',
  danger: 'stroke-red-500',
}

export function CircularProgress({
  value,
  max = 100,
  size = 40,
  strokeWidth = 4,
  variant = 'default',
  showValue = false,
  className,
}: CircularProgressProps) {
  const percent = Math.min(Math.max((value / max) * 100, 0), 100)
  const radius = (size - strokeWidth) / 2
  const circumference = radius * 2 * Math.PI
  const offset = circumference - (percent / 100) * circumference

  return (
    <div
      className={cn('relative inline-flex items-center justify-center', className)}
      style={{ width: size, height: size }}
    >
      <svg
        className="transform -rotate-90"
        width={size}
        height={size}
      >
        {/* Background circle */}
        <circle
          className="stroke-shadow-700"
          strokeWidth={strokeWidth}
          fill="none"
          r={radius}
          cx={size / 2}
          cy={size / 2}
        />
        {/* Progress circle */}
        <circle
          className={cn('transition-all duration-500 ease-out', circleVariants[variant])}
          strokeWidth={strokeWidth}
          strokeLinecap="round"
          fill="none"
          r={radius}
          cx={size / 2}
          cy={size / 2}
          style={{
            strokeDasharray: circumference,
            strokeDashoffset: offset,
          }}
        />
      </svg>
      {showValue && (
        <span className="absolute text-xs font-medium text-white">
          {percent.toFixed(0)}%
        </span>
      )}
    </div>
  )
}

