import React from 'react'
import { cn } from '../../utils/cn'

interface CardProps {
  children: React.ReactNode
  className?: string
  variant?: 'default' | 'ghost' | 'bordered' | 'gradient'
  hover?: boolean
  padding?: 'none' | 'sm' | 'md' | 'lg'
}

const variants = {
  default: 'bg-shadow-900/50 border border-shadow-700',
  ghost: 'bg-transparent',
  bordered: 'bg-transparent border-2 border-shadow-600',
  gradient: 'bg-gradient-to-br from-shadow-800/80 to-shadow-900/80 border border-shadow-600',
}

const paddings = {
  none: '',
  sm: 'p-3',
  md: 'p-4 sm:p-5',
  lg: 'p-6 sm:p-8',
}

export function Card({
  children,
  className,
  variant = 'default',
  hover = false,
  padding = 'md',
}: CardProps) {
  return (
    <div
      className={cn(
        'rounded-xl backdrop-blur-sm transition-all duration-200',
        variants[variant],
        paddings[padding],
        hover && 'hover:border-accent-500/50 hover:shadow-lg hover:shadow-accent-500/10',
        className
      )}
    >
      {children}
    </div>
  )
}

interface CardHeaderProps {
  children: React.ReactNode
  className?: string
  action?: React.ReactNode
}

export function CardHeader({ children, className, action }: CardHeaderProps) {
  return (
    <div className={cn('flex items-center justify-between mb-4', className)}>
      <div>{children}</div>
      {action && <div>{action}</div>}
    </div>
  )
}

interface CardTitleProps {
  children: React.ReactNode
  className?: string
  as?: 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6'
}

export function CardTitle({ children, className, as: Component = 'h3' }: CardTitleProps) {
  return (
    <Component className={cn('text-lg font-semibold text-white', className)}>
      {children}
    </Component>
  )
}

interface CardDescriptionProps {
  children: React.ReactNode
  className?: string
}

export function CardDescription({ children, className }: CardDescriptionProps) {
  return <p className={cn('text-shadow-400 text-sm mt-1', className)}>{children}</p>
}

interface CardContentProps {
  children: React.ReactNode
  className?: string
}

export function CardContent({ children, className }: CardContentProps) {
  return <div className={cn('', className)}>{children}</div>
}

interface CardFooterProps {
  children: React.ReactNode
  className?: string
}

export function CardFooter({ children, className }: CardFooterProps) {
  return (
    <div className={cn('mt-4 pt-4 border-t border-shadow-700 flex items-center gap-3', className)}>
      {children}
    </div>
  )
}


