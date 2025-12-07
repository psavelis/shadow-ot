'use client'

import React, { forwardRef } from 'react'
import { cn } from '../../utils/cn'
import { AlertCircle } from 'lucide-react'

export interface TextAreaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string
  error?: string
  hint?: string
  maxLength?: number
  showCount?: boolean
}

export const TextArea = forwardRef<HTMLTextAreaElement, TextAreaProps>(
  (
    { label, error, hint, maxLength, showCount, className, id, value, ...props },
    ref
  ) => {
    const textareaId = id || label?.toLowerCase().replace(/\s/g, '-')
    const charCount = typeof value === 'string' ? value.length : 0

    return (
      <div className="w-full">
        {label && (
          <label
            htmlFor={textareaId}
            className="block text-sm font-medium text-shadow-300 mb-1.5"
          >
            {label}
          </label>
        )}
        <div className="relative">
          <textarea
            ref={ref}
            id={textareaId}
            value={value}
            maxLength={maxLength}
            className={cn(
              'w-full px-4 py-3 rounded-lg border text-white placeholder:text-shadow-500',
              'bg-shadow-800 border-shadow-600',
              'transition-all duration-200 focus:outline-none focus:ring-2',
              'focus:border-accent-500 focus:ring-accent-500/30',
              'min-h-[100px] resize-y',
              error && 'border-red-500 focus:border-red-500 focus:ring-red-500/30',
              className
            )}
            {...props}
          />
        </div>
        <div className="flex items-center justify-between mt-1.5">
          <div>
            {error && (
              <p className="text-sm text-red-400 flex items-center gap-1">
                <AlertCircle className="h-4 w-4" />
                {error}
              </p>
            )}
            {hint && !error && (
              <p className="text-sm text-shadow-500">{hint}</p>
            )}
          </div>
          {showCount && maxLength && (
            <p className={cn(
              'text-sm',
              charCount >= maxLength ? 'text-red-400' : 'text-shadow-500'
            )}>
              {charCount}/{maxLength}
            </p>
          )}
        </div>
      </div>
    )
  }
)

TextArea.displayName = 'TextArea'


