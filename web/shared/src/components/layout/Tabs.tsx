'use client'

import React from 'react'
import * as TabsPrimitive from '@radix-ui/react-tabs'
import { cn } from '../../utils/cn'

interface TabsProps {
  defaultValue: string
  children: React.ReactNode
  className?: string
  onValueChange?: (value: string) => void
}

export function Tabs({ defaultValue, children, className, onValueChange }: TabsProps) {
  return (
    <TabsPrimitive.Root
      defaultValue={defaultValue}
      className={cn('w-full', className)}
      onValueChange={onValueChange}
    >
      {children}
    </TabsPrimitive.Root>
  )
}

interface TabsListProps {
  children: React.ReactNode
  className?: string
  variant?: 'default' | 'pills' | 'underline'
}

export function TabsList({ children, className, variant = 'default' }: TabsListProps) {
  return (
    <TabsPrimitive.List
      className={cn(
        'flex',
        variant === 'default' && 'bg-shadow-800 rounded-lg p-1 gap-1',
        variant === 'pills' && 'gap-2',
        variant === 'underline' && 'border-b border-shadow-700 gap-4',
        className
      )}
    >
      {children}
    </TabsPrimitive.List>
  )
}

interface TabsTriggerProps {
  value: string
  children: React.ReactNode
  className?: string
  disabled?: boolean
  variant?: 'default' | 'pills' | 'underline'
}

export function TabsTrigger({
  value,
  children,
  className,
  disabled,
  variant = 'default',
}: TabsTriggerProps) {
  return (
    <TabsPrimitive.Trigger
      value={value}
      disabled={disabled}
      className={cn(
        'transition-all text-sm font-medium focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed',
        variant === 'default' &&
          'px-4 py-2 rounded-md text-shadow-400 data-[state=active]:bg-shadow-700 data-[state=active]:text-white',
        variant === 'pills' &&
          'px-4 py-2 rounded-full text-shadow-400 hover:text-white data-[state=active]:bg-accent-500 data-[state=active]:text-white',
        variant === 'underline' &&
          'pb-3 text-shadow-400 border-b-2 border-transparent -mb-px data-[state=active]:text-accent-500 data-[state=active]:border-accent-500',
        className
      )}
    >
      {children}
    </TabsPrimitive.Trigger>
  )
}

interface TabsContentProps {
  value: string
  children: React.ReactNode
  className?: string
}

export function TabsContent({ value, children, className }: TabsContentProps) {
  return (
    <TabsPrimitive.Content
      value={value}
      className={cn(
        'mt-4 focus:outline-none data-[state=inactive]:hidden',
        className
      )}
    >
      {children}
    </TabsPrimitive.Content>
  )
}


