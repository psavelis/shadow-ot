'use client'

import React from 'react'
import { cn } from '../../utils/cn'
import { ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight } from 'lucide-react'

interface PaginationProps {
  currentPage: number
  totalPages: number
  onPageChange: (page: number) => void
  className?: string
  showFirstLast?: boolean
  siblingCount?: number
}

function generatePageNumbers(
  currentPage: number,
  totalPages: number,
  siblingCount: number
): (number | 'ellipsis')[] {
  const range = (start: number, end: number) =>
    Array.from({ length: end - start + 1 }, (_, i) => start + i)

  const totalNumbers = siblingCount * 2 + 3 // siblings + current + first + last
  const totalBlocks = totalNumbers + 2 // +2 for ellipsis

  if (totalPages <= totalBlocks) {
    return range(1, totalPages)
  }

  const leftSiblingIndex = Math.max(currentPage - siblingCount, 1)
  const rightSiblingIndex = Math.min(currentPage + siblingCount, totalPages)

  const shouldShowLeftEllipsis = leftSiblingIndex > 2
  const shouldShowRightEllipsis = rightSiblingIndex < totalPages - 1

  if (!shouldShowLeftEllipsis && shouldShowRightEllipsis) {
    const leftItemCount = 3 + 2 * siblingCount
    const leftRange = range(1, leftItemCount)
    return [...leftRange, 'ellipsis', totalPages]
  }

  if (shouldShowLeftEllipsis && !shouldShowRightEllipsis) {
    const rightItemCount = 3 + 2 * siblingCount
    const rightRange = range(totalPages - rightItemCount + 1, totalPages)
    return [1, 'ellipsis', ...rightRange]
  }

  const middleRange = range(leftSiblingIndex, rightSiblingIndex)
  return [1, 'ellipsis', ...middleRange, 'ellipsis', totalPages]
}

export function Pagination({
  currentPage,
  totalPages,
  onPageChange,
  className,
  showFirstLast = true,
  siblingCount = 1,
}: PaginationProps) {
  if (totalPages <= 1) return null

  const pageNumbers = generatePageNumbers(currentPage, totalPages, siblingCount)

  return (
    <nav className={cn('flex items-center justify-center gap-1', className)}>
      {showFirstLast && (
        <button
          onClick={() => onPageChange(1)}
          disabled={currentPage === 1}
          className="p-2 rounded-lg text-shadow-400 hover:text-white hover:bg-shadow-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          aria-label="First page"
        >
          <ChevronsLeft className="w-4 h-4" />
        </button>
      )}
      <button
        onClick={() => onPageChange(currentPage - 1)}
        disabled={currentPage === 1}
        className="p-2 rounded-lg text-shadow-400 hover:text-white hover:bg-shadow-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        aria-label="Previous page"
      >
        <ChevronLeft className="w-4 h-4" />
      </button>

      {pageNumbers.map((pageNumber, index) =>
        pageNumber === 'ellipsis' ? (
          <span
            key={`ellipsis-${index}`}
            className="px-3 py-2 text-shadow-500"
          >
            ...
          </span>
        ) : (
          <button
            key={pageNumber}
            onClick={() => onPageChange(pageNumber)}
            className={cn(
              'min-w-[40px] h-10 rounded-lg text-sm font-medium transition-colors',
              currentPage === pageNumber
                ? 'bg-accent-500 text-white'
                : 'text-shadow-400 hover:text-white hover:bg-shadow-700'
            )}
          >
            {pageNumber}
          </button>
        )
      )}

      <button
        onClick={() => onPageChange(currentPage + 1)}
        disabled={currentPage === totalPages}
        className="p-2 rounded-lg text-shadow-400 hover:text-white hover:bg-shadow-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        aria-label="Next page"
      >
        <ChevronRight className="w-4 h-4" />
      </button>
      {showFirstLast && (
        <button
          onClick={() => onPageChange(totalPages)}
          disabled={currentPage === totalPages}
          className="p-2 rounded-lg text-shadow-400 hover:text-white hover:bg-shadow-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          aria-label="Last page"
        >
          <ChevronsRight className="w-4 h-4" />
        </button>
      )}
    </nav>
  )
}

// Simple pagination info
interface PaginationInfoProps {
  currentPage: number
  pageSize: number
  total: number
  className?: string
}

export function PaginationInfo({
  currentPage,
  pageSize,
  total,
  className,
}: PaginationInfoProps) {
  const start = (currentPage - 1) * pageSize + 1
  const end = Math.min(currentPage * pageSize, total)

  return (
    <p className={cn('text-sm text-shadow-400', className)}>
      Showing <span className="text-white font-medium">{start}</span> to{' '}
      <span className="text-white font-medium">{end}</span> of{' '}
      <span className="text-white font-medium">{total.toLocaleString()}</span> results
    </p>
  )
}

