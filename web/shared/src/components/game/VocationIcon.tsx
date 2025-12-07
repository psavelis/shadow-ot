import React from 'react'
import { cn } from '../../utils/cn'
import { Shield, Target, Wand2, Leaf, User } from 'lucide-react'
import { Vocation } from '../../types'

interface VocationIconProps {
  vocation: Vocation
  size?: 'sm' | 'md' | 'lg'
  className?: string
  showLabel?: boolean
}

const vocationConfig: Record<Vocation, { icon: typeof Shield; color: string; abbr: string }> = {
  'None': { icon: User, color: 'text-gray-400', abbr: 'N' },
  'Knight': { icon: Shield, color: 'text-red-400', abbr: 'K' },
  'Elite Knight': { icon: Shield, color: 'text-red-500', abbr: 'EK' },
  'Paladin': { icon: Target, color: 'text-yellow-400', abbr: 'P' },
  'Royal Paladin': { icon: Target, color: 'text-yellow-500', abbr: 'RP' },
  'Sorcerer': { icon: Wand2, color: 'text-blue-400', abbr: 'S' },
  'Master Sorcerer': { icon: Wand2, color: 'text-blue-500', abbr: 'MS' },
  'Druid': { icon: Leaf, color: 'text-green-400', abbr: 'D' },
  'Elder Druid': { icon: Leaf, color: 'text-green-500', abbr: 'ED' },
}

const sizes = {
  sm: 'w-4 h-4',
  md: 'w-5 h-5',
  lg: 'w-6 h-6',
}

export function VocationIcon({ vocation, size = 'md', className, showLabel }: VocationIconProps) {
  const config = vocationConfig[vocation]
  const Icon = config.icon

  if (showLabel) {
    return (
      <span className={cn('inline-flex items-center gap-1.5', className)}>
        <Icon className={cn(sizes[size], config.color)} />
        <span className={cn('text-sm', config.color)}>{config.abbr}</span>
      </span>
    )
  }

  return <Icon className={cn(sizes[size], config.color, className)} />
}

export function getVocationColor(vocation: Vocation): string {
  return vocationConfig[vocation]?.color || 'text-gray-400'
}

export function getVocationAbbr(vocation: Vocation): string {
  return vocationConfig[vocation]?.abbr || 'N'
}


