'use client'

import React from 'react'
import * as DropdownMenu from '@radix-ui/react-dropdown-menu'
import { Check, ChevronRight, Circle } from 'lucide-react'
import { cn } from '../../utils/cn'

interface DropdownProps {
  trigger: React.ReactNode
  children: React.ReactNode
  align?: 'start' | 'center' | 'end'
  side?: 'top' | 'bottom' | 'left' | 'right'
}

export function Dropdown({ trigger, children, align = 'end', side = 'bottom' }: DropdownProps) {
  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>{trigger}</DropdownMenu.Trigger>
      <DropdownMenu.Portal>
        <DropdownMenu.Content
          align={align}
          side={side}
          sideOffset={8}
          className={cn(
            'z-50 min-w-[180px] overflow-hidden rounded-lg',
            'bg-shadow-800 border border-shadow-600 shadow-xl',
            'data-[state=open]:animate-in data-[state=closed]:animate-out',
            'data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0',
            'data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95',
            'data-[side=bottom]:slide-in-from-top-2 data-[side=top]:slide-in-from-bottom-2'
          )}
        >
          <div className="p-1">{children}</div>
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  )
}

interface DropdownItemProps {
  children: React.ReactNode
  icon?: React.ReactNode
  shortcut?: string
  disabled?: boolean
  danger?: boolean
  onSelect?: () => void
}

export function DropdownItem({
  children,
  icon,
  shortcut,
  disabled,
  danger,
  onSelect,
}: DropdownItemProps) {
  return (
    <DropdownMenu.Item
      disabled={disabled}
      onSelect={onSelect}
      className={cn(
        'flex items-center gap-2 px-3 py-2 rounded-md text-sm cursor-pointer outline-none',
        'transition-colors',
        danger
          ? 'text-red-400 data-[highlighted]:bg-red-500/20'
          : 'text-white data-[highlighted]:bg-shadow-700',
        disabled && 'opacity-50 cursor-not-allowed'
      )}
    >
      {icon && <span className="w-4 h-4">{icon}</span>}
      <span className="flex-1">{children}</span>
      {shortcut && (
        <span className="ml-auto text-xs text-shadow-500">{shortcut}</span>
      )}
    </DropdownMenu.Item>
  )
}

export function DropdownSeparator() {
  return <DropdownMenu.Separator className="h-px my-1 bg-shadow-600" />
}

interface DropdownLabelProps {
  children: React.ReactNode
}

export function DropdownLabel({ children }: DropdownLabelProps) {
  return (
    <DropdownMenu.Label className="px-3 py-1.5 text-xs font-semibold text-shadow-500 uppercase tracking-wider">
      {children}
    </DropdownMenu.Label>
  )
}

interface DropdownCheckboxItemProps {
  children: React.ReactNode
  checked: boolean
  onCheckedChange: (checked: boolean) => void
}

export function DropdownCheckboxItem({
  children,
  checked,
  onCheckedChange,
}: DropdownCheckboxItemProps) {
  return (
    <DropdownMenu.CheckboxItem
      checked={checked}
      onCheckedChange={onCheckedChange}
      className={cn(
        'flex items-center gap-2 px-3 py-2 rounded-md text-sm cursor-pointer outline-none',
        'text-white data-[highlighted]:bg-shadow-700 transition-colors'
      )}
    >
      <div className="w-4 h-4 rounded border border-shadow-500 flex items-center justify-center">
        <DropdownMenu.ItemIndicator>
          <Check className="w-3 h-3 text-accent-500" />
        </DropdownMenu.ItemIndicator>
      </div>
      {children}
    </DropdownMenu.CheckboxItem>
  )
}

interface DropdownSubProps {
  trigger: React.ReactNode
  children: React.ReactNode
}

export function DropdownSub({ trigger, children }: DropdownSubProps) {
  return (
    <DropdownMenu.Sub>
      <DropdownMenu.SubTrigger
        className={cn(
          'flex items-center gap-2 px-3 py-2 rounded-md text-sm cursor-pointer outline-none',
          'text-white data-[highlighted]:bg-shadow-700 transition-colors'
        )}
      >
        {trigger}
        <ChevronRight className="ml-auto w-4 h-4" />
      </DropdownMenu.SubTrigger>
      <DropdownMenu.Portal>
        <DropdownMenu.SubContent
          sideOffset={4}
          className={cn(
            'z-50 min-w-[180px] overflow-hidden rounded-lg',
            'bg-shadow-800 border border-shadow-600 shadow-xl p-1',
            'data-[state=open]:animate-in data-[state=closed]:animate-out',
            'data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0',
            'data-[side=right]:slide-in-from-left-2'
          )}
        >
          {children}
        </DropdownMenu.SubContent>
      </DropdownMenu.Portal>
    </DropdownMenu.Sub>
  )
}


