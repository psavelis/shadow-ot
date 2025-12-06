import React from 'react'
import { cn } from '../../utils/cn'
import { ChevronRight, Home } from 'lucide-react'

interface BreadcrumbItem {
  label: string
  href?: string
  icon?: React.ReactNode
}

interface BreadcrumbProps {
  items: BreadcrumbItem[]
  className?: string
  separator?: React.ReactNode
  showHome?: boolean
  homeHref?: string
}

export function Breadcrumb({
  items,
  className,
  separator = <ChevronRight className="w-4 h-4 text-shadow-600" />,
  showHome = true,
  homeHref = '/',
}: BreadcrumbProps) {
  const allItems = showHome
    ? [{ label: 'Home', href: homeHref, icon: <Home className="w-4 h-4" /> }, ...items]
    : items

  return (
    <nav className={cn('flex items-center', className)} aria-label="Breadcrumb">
      <ol className="flex items-center space-x-2">
        {allItems.map((item, index) => {
          const isLast = index === allItems.length - 1

          return (
            <li key={index} className="flex items-center">
              {index > 0 && <span className="mx-2">{separator}</span>}
              {isLast ? (
                <span className="flex items-center gap-1.5 text-sm text-white font-medium">
                  {item.icon}
                  {item.label}
                </span>
              ) : (
                <a
                  href={item.href}
                  className="flex items-center gap-1.5 text-sm text-shadow-400 hover:text-white transition-colors"
                >
                  {item.icon}
                  {item.label}
                </a>
              )}
            </li>
          )
        })}
      </ol>
    </nav>
  )
}

