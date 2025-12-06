import React from 'react'
import { cn } from '../../utils/cn'
import { TrendingUp, TrendingDown, Minus } from 'lucide-react'

interface StatProps {
  label: string
  value: string | number
  change?: {
    value: string | number
    type: 'increase' | 'decrease' | 'neutral'
  }
  icon?: React.ReactNode
  className?: string
  size?: 'sm' | 'md' | 'lg'
}

const sizeStyles = {
  sm: {
    value: 'text-lg font-bold',
    label: 'text-xs',
    change: 'text-xs',
    icon: 'w-8 h-8',
    iconInner: 'w-4 h-4',
  },
  md: {
    value: 'text-2xl font-bold',
    label: 'text-sm',
    change: 'text-sm',
    icon: 'w-10 h-10',
    iconInner: 'w-5 h-5',
  },
  lg: {
    value: 'text-3xl font-bold',
    label: 'text-base',
    change: 'text-base',
    icon: 'w-12 h-12',
    iconInner: 'w-6 h-6',
  },
}

export function Stat({
  label,
  value,
  change,
  icon,
  className,
  size = 'md',
}: StatProps) {
  const styles = sizeStyles[size]

  return (
    <div className={cn('', className)}>
      <div className="flex items-center justify-between mb-3">
        {icon && (
          <div
            className={cn(
              'rounded-lg bg-accent-500/10 flex items-center justify-center',
              styles.icon
            )}
          >
            {React.cloneElement(icon as React.ReactElement, {
              className: cn(styles.iconInner, 'text-accent-500'),
            })}
          </div>
        )}
        {change && (
          <span
            className={cn(
              'flex items-center font-medium',
              styles.change,
              change.type === 'increase' && 'text-green-500',
              change.type === 'decrease' && 'text-red-500',
              change.type === 'neutral' && 'text-shadow-400'
            )}
          >
            {change.type === 'increase' && <TrendingUp className="w-4 h-4 mr-1" />}
            {change.type === 'decrease' && <TrendingDown className="w-4 h-4 mr-1" />}
            {change.type === 'neutral' && <Minus className="w-4 h-4 mr-1" />}
            {change.value}
          </span>
        )}
      </div>
      <p className={cn('text-white', styles.value)}>{value}</p>
      <p className={cn('text-shadow-400 mt-1', styles.label)}>{label}</p>
    </div>
  )
}

interface StatCardProps extends StatProps {
  variant?: 'default' | 'bordered' | 'gradient'
}

export function StatCard({ variant = 'default', ...props }: StatCardProps) {
  const variants = {
    default: 'bg-shadow-900/50 border border-shadow-700',
    bordered: 'bg-transparent border-2 border-shadow-600',
    gradient: 'bg-gradient-to-br from-shadow-800/80 to-shadow-900/80 border border-shadow-600',
  }

  return (
    <div className={cn('p-4 sm:p-5 rounded-xl', variants[variant])}>
      <Stat {...props} />
    </div>
  )
}

interface StatGridProps {
  children: React.ReactNode
  columns?: 2 | 3 | 4
  className?: string
}

export function StatGrid({ children, columns = 4, className }: StatGridProps) {
  const gridCols = {
    2: 'sm:grid-cols-2',
    3: 'sm:grid-cols-2 lg:grid-cols-3',
    4: 'sm:grid-cols-2 lg:grid-cols-4',
  }

  return (
    <div className={cn('grid gap-4', gridCols[columns], className)}>
      {children}
    </div>
  )
}

