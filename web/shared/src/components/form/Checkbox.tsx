'use client'

import React, { forwardRef } from 'react'
import * as CheckboxPrimitive from '@radix-ui/react-checkbox'
import { Check, Minus } from 'lucide-react'
import { cn } from '../../utils/cn'

interface CheckboxProps {
  checked?: boolean | 'indeterminate'
  onCheckedChange?: (checked: boolean | 'indeterminate') => void
  label?: string
  description?: string
  error?: string
  disabled?: boolean
  className?: string
  id?: string
}

export const Checkbox = forwardRef<HTMLButtonElement, CheckboxProps>(
  (
    { checked, onCheckedChange, label, description, error, disabled, className, id },
    ref
  ) => {
    const checkboxId = id || label?.toLowerCase().replace(/\s/g, '-')

    return (
      <div className={cn('flex items-start', className)}>
        <CheckboxPrimitive.Root
          ref={ref}
          id={checkboxId}
          checked={checked}
          onCheckedChange={onCheckedChange}
          disabled={disabled}
          className={cn(
            'h-5 w-5 shrink-0 rounded border-2 transition-all duration-200',
            'focus:outline-none focus:ring-2 focus:ring-accent-500/30 focus:ring-offset-2 focus:ring-offset-shadow-900',
            'disabled:opacity-50 disabled:cursor-not-allowed',
            'border-shadow-500 bg-shadow-800',
            'data-[state=checked]:bg-accent-500 data-[state=checked]:border-accent-500',
            'data-[state=indeterminate]:bg-accent-500 data-[state=indeterminate]:border-accent-500',
            error && 'border-red-500'
          )}
        >
          <CheckboxPrimitive.Indicator className="flex items-center justify-center text-white">
            {checked === 'indeterminate' ? (
              <Minus className="h-3.5 w-3.5" />
            ) : (
              <Check className="h-3.5 w-3.5" />
            )}
          </CheckboxPrimitive.Indicator>
        </CheckboxPrimitive.Root>
        {(label || description) && (
          <div className="ml-3">
            {label && (
              <label
                htmlFor={checkboxId}
                className={cn(
                  'text-sm font-medium text-white cursor-pointer',
                  disabled && 'cursor-not-allowed opacity-50'
                )}
              >
                {label}
              </label>
            )}
            {description && (
              <p className="text-sm text-shadow-400 mt-0.5">{description}</p>
            )}
            {error && <p className="text-sm text-red-400 mt-0.5">{error}</p>}
          </div>
        )}
      </div>
    )
  }
)

Checkbox.displayName = 'Checkbox'


