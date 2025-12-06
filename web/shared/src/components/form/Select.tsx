'use client'

import React from 'react'
import * as SelectPrimitive from '@radix-ui/react-select'
import { Check, ChevronDown, ChevronUp } from 'lucide-react'
import { cn } from '../../utils/cn'

interface SelectProps {
  value?: string
  onValueChange?: (value: string) => void
  placeholder?: string
  label?: string
  error?: string
  children: React.ReactNode
  className?: string
  disabled?: boolean
}

export function Select({
  value,
  onValueChange,
  placeholder = 'Select...',
  label,
  error,
  children,
  className,
  disabled,
}: SelectProps) {
  return (
    <div className={cn('w-full', className)}>
      {label && (
        <label className="block text-sm font-medium text-shadow-300 mb-1.5">
          {label}
        </label>
      )}
      <SelectPrimitive.Root value={value} onValueChange={onValueChange} disabled={disabled}>
        <SelectPrimitive.Trigger
          className={cn(
            'flex items-center justify-between w-full px-4 py-2.5 rounded-lg',
            'bg-shadow-800 border border-shadow-600 text-white',
            'focus:outline-none focus:ring-2 focus:ring-accent-500/30 focus:border-accent-500',
            'transition-all duration-200',
            'data-[placeholder]:text-shadow-500',
            disabled && 'opacity-50 cursor-not-allowed',
            error && 'border-red-500'
          )}
        >
          <SelectPrimitive.Value placeholder={placeholder} />
          <SelectPrimitive.Icon>
            <ChevronDown className="h-4 w-4 text-shadow-400" />
          </SelectPrimitive.Icon>
        </SelectPrimitive.Trigger>
        <SelectPrimitive.Portal>
          <SelectPrimitive.Content
            className={cn(
              'z-50 overflow-hidden bg-shadow-800 border border-shadow-600 rounded-lg shadow-xl',
              'data-[state=open]:animate-in data-[state=closed]:animate-out',
              'data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0',
              'data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95',
              'data-[side=bottom]:slide-in-from-top-2 data-[side=top]:slide-in-from-bottom-2'
            )}
            position="popper"
            sideOffset={4}
          >
            <SelectPrimitive.ScrollUpButton className="flex items-center justify-center h-6 bg-shadow-800 cursor-default">
              <ChevronUp className="h-4 w-4 text-shadow-400" />
            </SelectPrimitive.ScrollUpButton>
            <SelectPrimitive.Viewport className="p-1">
              {children}
            </SelectPrimitive.Viewport>
            <SelectPrimitive.ScrollDownButton className="flex items-center justify-center h-6 bg-shadow-800 cursor-default">
              <ChevronDown className="h-4 w-4 text-shadow-400" />
            </SelectPrimitive.ScrollDownButton>
          </SelectPrimitive.Content>
        </SelectPrimitive.Portal>
      </SelectPrimitive.Root>
      {error && <p className="mt-1.5 text-sm text-red-400">{error}</p>}
    </div>
  )
}

interface SelectItemProps {
  value: string
  children: React.ReactNode
  disabled?: boolean
  className?: string
}

export function SelectItem({ value, children, disabled, className }: SelectItemProps) {
  return (
    <SelectPrimitive.Item
      value={value}
      disabled={disabled}
      className={cn(
        'relative flex items-center px-3 py-2 rounded-md text-sm text-white cursor-pointer select-none',
        'outline-none data-[highlighted]:bg-shadow-700 data-[highlighted]:text-white',
        'data-[disabled]:opacity-50 data-[disabled]:pointer-events-none',
        className
      )}
    >
      <SelectPrimitive.ItemText>{children}</SelectPrimitive.ItemText>
      <SelectPrimitive.ItemIndicator className="absolute right-2">
        <Check className="h-4 w-4 text-accent-500" />
      </SelectPrimitive.ItemIndicator>
    </SelectPrimitive.Item>
  )
}

interface SelectGroupProps {
  label: string
  children: React.ReactNode
}

export function SelectGroup({ label, children }: SelectGroupProps) {
  return (
    <SelectPrimitive.Group>
      <SelectPrimitive.Label className="px-3 py-1.5 text-xs font-semibold text-shadow-500 uppercase tracking-wider">
        {label}
      </SelectPrimitive.Label>
      {children}
    </SelectPrimitive.Group>
  )
}

export function SelectSeparator() {
  return <SelectPrimitive.Separator className="h-px my-1 bg-shadow-600" />
}

