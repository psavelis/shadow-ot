'use client'

import React, { Fragment } from 'react'
import * as Dialog from '@radix-ui/react-dialog'
import { X } from 'lucide-react'
import { cn } from '../../utils/cn'

interface ModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  children: React.ReactNode
  title?: string
  description?: string
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full'
  className?: string
  showClose?: boolean
}

const sizes = {
  sm: 'max-w-md',
  md: 'max-w-lg',
  lg: 'max-w-2xl',
  xl: 'max-w-4xl',
  full: 'max-w-[90vw]',
}

export function Modal({
  open,
  onOpenChange,
  children,
  title,
  description,
  size = 'md',
  className,
  showClose = true,
}: ModalProps) {
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0" />
        <Dialog.Content
          className={cn(
            'fixed left-[50%] top-[50%] z-50 w-full translate-x-[-50%] translate-y-[-50%]',
            'bg-shadow-900 border border-shadow-700 rounded-xl shadow-2xl',
            'p-6 focus:outline-none',
            'data-[state=open]:animate-in data-[state=closed]:animate-out',
            'data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0',
            'data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95',
            'data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%]',
            'data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%]',
            sizes[size],
            className
          )}
        >
          {(title || showClose) && (
            <div className="flex items-start justify-between mb-4">
              <div>
                {title && (
                  <Dialog.Title className="text-xl font-display font-bold text-white">
                    {title}
                  </Dialog.Title>
                )}
                {description && (
                  <Dialog.Description className="text-shadow-400 text-sm mt-1">
                    {description}
                  </Dialog.Description>
                )}
              </div>
              {showClose && (
                <Dialog.Close asChild>
                  <button
                    className="text-shadow-400 hover:text-white transition-colors p-1 rounded-lg hover:bg-shadow-700"
                    aria-label="Close"
                  >
                    <X className="h-5 w-5" />
                  </button>
                </Dialog.Close>
              )}
            </div>
          )}
          {children}
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  )
}

interface ModalTriggerProps {
  children: React.ReactNode
  asChild?: boolean
}

export function ModalTrigger({ children, asChild }: ModalTriggerProps) {
  return <Dialog.Trigger asChild={asChild}>{children}</Dialog.Trigger>
}

interface ModalFooterProps {
  children: React.ReactNode
  className?: string
}

export function ModalFooter({ children, className }: ModalFooterProps) {
  return (
    <div className={cn('mt-6 flex items-center justify-end gap-3', className)}>
      {children}
    </div>
  )
}

export function ModalClose({ children }: { children: React.ReactNode }) {
  return <Dialog.Close asChild>{children}</Dialog.Close>
}

