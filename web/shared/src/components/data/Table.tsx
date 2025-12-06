import React from 'react'
import { cn } from '../../utils/cn'
import { ChevronUp, ChevronDown, ChevronsUpDown } from 'lucide-react'

interface TableProps {
  children: React.ReactNode
  className?: string
}

export function Table({ children, className }: TableProps) {
  return (
    <div className={cn('overflow-x-auto', className)}>
      <table className="w-full">{children}</table>
    </div>
  )
}

interface TableHeaderProps {
  children: React.ReactNode
  className?: string
}

export function TableHeader({ children, className }: TableHeaderProps) {
  return <thead className={cn('', className)}>{children}</thead>
}

interface TableBodyProps {
  children: React.ReactNode
  className?: string
}

export function TableBody({ children, className }: TableBodyProps) {
  return <tbody className={cn('', className)}>{children}</tbody>
}

interface TableRowProps {
  children: React.ReactNode
  className?: string
  hover?: boolean
  onClick?: () => void
}

export function TableRow({ children, className, hover = true, onClick }: TableRowProps) {
  return (
    <tr
      onClick={onClick}
      className={cn(
        'border-b border-shadow-800 last:border-0',
        hover && 'hover:bg-shadow-800/50',
        onClick && 'cursor-pointer',
        className
      )}
    >
      {children}
    </tr>
  )
}

interface TableHeadProps {
  children: React.ReactNode
  className?: string
  sortable?: boolean
  sorted?: 'asc' | 'desc' | false
  onSort?: () => void
  align?: 'left' | 'center' | 'right'
}

export function TableHead({
  children,
  className,
  sortable = false,
  sorted = false,
  onSort,
  align = 'left',
}: TableHeadProps) {
  const alignClass = {
    left: 'text-left',
    center: 'text-center',
    right: 'text-right',
  }

  return (
    <th
      onClick={sortable ? onSort : undefined}
      className={cn(
        'py-3 px-4 text-shadow-400 font-medium text-sm',
        alignClass[align],
        sortable && 'cursor-pointer hover:text-white select-none',
        className
      )}
    >
      <div className="flex items-center gap-1">
        {children}
        {sortable && (
          <span className="ml-1">
            {sorted === 'asc' ? (
              <ChevronUp className="w-4 h-4" />
            ) : sorted === 'desc' ? (
              <ChevronDown className="w-4 h-4" />
            ) : (
              <ChevronsUpDown className="w-4 h-4 text-shadow-600" />
            )}
          </span>
        )}
      </div>
    </th>
  )
}

interface TableCellProps {
  children: React.ReactNode
  className?: string
  align?: 'left' | 'center' | 'right'
}

export function TableCell({ children, className, align = 'left' }: TableCellProps) {
  const alignClass = {
    left: 'text-left',
    center: 'text-center',
    right: 'text-right',
  }

  return (
    <td className={cn('py-3 px-4', alignClass[align], className)}>{children}</td>
  )
}

// Empty state for tables
interface TableEmptyProps {
  message?: string
  description?: string
  action?: React.ReactNode
  icon?: React.ReactNode
}

export function TableEmpty({
  message = 'No data found',
  description,
  action,
  icon,
}: TableEmptyProps) {
  return (
    <TableRow hover={false}>
      <TableCell className="py-12" colSpan={100}>
        <div className="flex flex-col items-center justify-center text-center">
          {icon && <div className="mb-4 text-shadow-500">{icon}</div>}
          <p className="text-white font-medium">{message}</p>
          {description && (
            <p className="text-shadow-500 text-sm mt-1">{description}</p>
          )}
          {action && <div className="mt-4">{action}</div>}
        </div>
      </TableCell>
    </TableRow>
  )
}

