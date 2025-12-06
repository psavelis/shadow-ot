import React from 'react'
import { cn } from '../../utils/cn'
import { AlertCircle, CheckCircle, AlertTriangle, Info, X } from 'lucide-react'

interface AlertProps {
  title?: string
  children: React.ReactNode
  variant?: 'success' | 'error' | 'warning' | 'info'
  className?: string
  onClose?: () => void
  icon?: React.ReactNode
}

const variants = {
  success: {
    container: 'bg-green-500/10 border-green-500/30',
    icon: 'text-green-400',
    title: 'text-green-400',
    content: 'text-green-300/80',
    defaultIcon: CheckCircle,
  },
  error: {
    container: 'bg-red-500/10 border-red-500/30',
    icon: 'text-red-400',
    title: 'text-red-400',
    content: 'text-red-300/80',
    defaultIcon: AlertCircle,
  },
  warning: {
    container: 'bg-yellow-500/10 border-yellow-500/30',
    icon: 'text-yellow-400',
    title: 'text-yellow-400',
    content: 'text-yellow-300/80',
    defaultIcon: AlertTriangle,
  },
  info: {
    container: 'bg-blue-500/10 border-blue-500/30',
    icon: 'text-blue-400',
    title: 'text-blue-400',
    content: 'text-blue-300/80',
    defaultIcon: Info,
  },
}

export function Alert({
  title,
  children,
  variant = 'info',
  className,
  onClose,
  icon,
}: AlertProps) {
  const styles = variants[variant]
  const Icon = styles.defaultIcon

  return (
    <div
      className={cn(
        'flex gap-3 p-4 rounded-lg border',
        styles.container,
        className
      )}
      role="alert"
    >
      <div className={cn('flex-shrink-0', styles.icon)}>
        {icon || <Icon className="w-5 h-5" />}
      </div>
      <div className="flex-1 min-w-0">
        {title && (
          <p className={cn('font-medium', styles.title)}>{title}</p>
        )}
        <div className={cn('text-sm', title && 'mt-1', styles.content)}>
          {children}
        </div>
      </div>
      {onClose && (
        <button
          onClick={onClose}
          className={cn(
            'flex-shrink-0 p-1 rounded hover:bg-white/10 transition-colors',
            styles.icon
          )}
        >
          <X className="w-4 h-4" />
        </button>
      )}
    </div>
  )
}

// Inline alert (simpler version)
interface InlineAlertProps {
  children: React.ReactNode
  variant?: 'success' | 'error' | 'warning' | 'info'
  className?: string
}

export function InlineAlert({ children, variant = 'info', className }: InlineAlertProps) {
  const styles = variants[variant]
  const Icon = styles.defaultIcon

  return (
    <p className={cn('flex items-center gap-2 text-sm', styles.content, className)}>
      <Icon className={cn('w-4 h-4', styles.icon)} />
      {children}
    </p>
  )
}

