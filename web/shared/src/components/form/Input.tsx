'use client'

import React, { forwardRef } from 'react'
import { cn } from '../../utils/cn'
import { AlertCircle, Eye, EyeOff, Search } from 'lucide-react'

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string
  error?: string
  hint?: string
  leftIcon?: React.ReactNode
  rightIcon?: React.ReactNode
  variant?: 'default' | 'filled' | 'ghost'
}

export const Input = forwardRef<HTMLInputElement, InputProps>(
  (
    {
      label,
      error,
      hint,
      leftIcon,
      rightIcon,
      className,
      type,
      variant = 'default',
      id,
      ...props
    },
    ref
  ) => {
    const [showPassword, setShowPassword] = React.useState(false)
    const inputId = id || label?.toLowerCase().replace(/\s/g, '-')
    const isPassword = type === 'password'

    const variants = {
      default:
        'bg-shadow-800 border-shadow-600 focus:border-accent-500 focus:ring-accent-500/30',
      filled:
        'bg-shadow-700 border-transparent focus:bg-shadow-800 focus:border-accent-500 focus:ring-accent-500/30',
      ghost:
        'bg-transparent border-transparent hover:bg-shadow-800 focus:bg-shadow-800 focus:border-accent-500',
    }

    return (
      <div className="w-full">
        {label && (
          <label
            htmlFor={inputId}
            className="block text-sm font-medium text-shadow-300 mb-1.5"
          >
            {label}
          </label>
        )}
        <div className="relative">
          {leftIcon && (
            <div className="absolute left-3 top-1/2 -translate-y-1/2 text-shadow-500">
              {leftIcon}
            </div>
          )}
          <input
            ref={ref}
            id={inputId}
            type={isPassword && showPassword ? 'text' : type}
            className={cn(
              'w-full px-4 py-2.5 rounded-lg border text-white placeholder:text-shadow-500',
              'transition-all duration-200 focus:outline-none focus:ring-2',
              variants[variant],
              leftIcon && 'pl-10',
              (rightIcon || isPassword) && 'pr-10',
              error && 'border-red-500 focus:border-red-500 focus:ring-red-500/30',
              className
            )}
            {...props}
          />
          {isPassword && (
            <button
              type="button"
              onClick={() => setShowPassword(!showPassword)}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-shadow-500 hover:text-shadow-300 transition-colors"
            >
              {showPassword ? (
                <EyeOff className="h-5 w-5" />
              ) : (
                <Eye className="h-5 w-5" />
              )}
            </button>
          )}
          {rightIcon && !isPassword && (
            <div className="absolute right-3 top-1/2 -translate-y-1/2 text-shadow-500">
              {rightIcon}
            </div>
          )}
        </div>
        {error && (
          <p className="mt-1.5 text-sm text-red-400 flex items-center gap-1">
            <AlertCircle className="h-4 w-4" />
            {error}
          </p>
        )}
        {hint && !error && (
          <p className="mt-1.5 text-sm text-shadow-500">{hint}</p>
        )}
      </div>
    )
  }
)

Input.displayName = 'Input'

// Search Input variant
export const SearchInput = forwardRef<HTMLInputElement, Omit<InputProps, 'leftIcon'>>(
  ({ className, ...props }, ref) => {
    return (
      <Input
        ref={ref}
        type="search"
        leftIcon={<Search className="h-5 w-5" />}
        className={cn('', className)}
        {...props}
      />
    )
  }
)

SearchInput.displayName = 'SearchInput'


