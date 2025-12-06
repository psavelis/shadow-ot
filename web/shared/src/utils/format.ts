import { formatDistanceToNow, format, parseISO } from 'date-fns'

/**
 * Format a date to relative time (e.g., "2 hours ago")
 */
export function formatRelativeTime(date: string | Date): string {
  const parsed = typeof date === 'string' ? parseISO(date) : date
  return formatDistanceToNow(parsed, { addSuffix: true })
}

/**
 * Format a date to a readable string
 */
export function formatDate(date: string | Date, formatStr = 'MMM d, yyyy'): string {
  const parsed = typeof date === 'string' ? parseISO(date) : date
  return format(parsed, formatStr)
}

/**
 * Format a date with time
 */
export function formatDateTime(date: string | Date): string {
  const parsed = typeof date === 'string' ? parseISO(date) : date
  return format(parsed, 'MMM d, yyyy HH:mm')
}

/**
 * Format a number with commas (e.g., 1,234,567)
 */
export function formatNumber(value: number): string {
  return value.toLocaleString()
}

/**
 * Format experience with suffix (e.g., 1.2M, 500K)
 */
export function formatExperience(value: number): string {
  if (value >= 1000000000) {
    return `${(value / 1000000000).toFixed(2)}B`
  }
  if (value >= 1000000) {
    return `${(value / 1000000).toFixed(2)}M`
  }
  if (value >= 1000) {
    return `${(value / 1000).toFixed(1)}K`
  }
  return value.toString()
}

/**
 * Format gold coins
 */
export function formatGold(value: number): string {
  if (value >= 1000000000) {
    return `${(value / 1000000000).toFixed(2)}kkk`
  }
  if (value >= 1000000) {
    return `${(value / 1000000).toFixed(2)}kk`
  }
  if (value >= 1000) {
    return `${(value / 1000).toFixed(0)}k`
  }
  return value.toLocaleString()
}

/**
 * Format level with leading zeros
 */
export function formatLevel(level: number, digits = 3): string {
  return level.toString().padStart(digits, '0')
}

/**
 * Format percentage
 */
export function formatPercent(value: number, decimals = 1): string {
  return `${value.toFixed(decimals)}%`
}

/**
 * Format skill value with percentage
 */
export function formatSkill(level: number, percent: number): string {
  return `${level} (${percent.toFixed(2)}%)`
}

/**
 * Format duration in human readable format
 */
export function formatDuration(seconds: number): string {
  const days = Math.floor(seconds / 86400)
  const hours = Math.floor((seconds % 86400) / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)

  if (days > 0) {
    return `${days}d ${hours}h`
  }
  if (hours > 0) {
    return `${hours}h ${minutes}m`
  }
  return `${minutes}m`
}

/**
 * Format uptime duration
 */
export function formatUptime(seconds: number): string {
  const days = Math.floor(seconds / 86400)
  const hours = Math.floor((seconds % 86400) / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)

  const parts = []
  if (days > 0) parts.push(`${days} day${days !== 1 ? 's' : ''}`)
  if (hours > 0) parts.push(`${hours} hour${hours !== 1 ? 's' : ''}`)
  if (minutes > 0 && days === 0) parts.push(`${minutes} minute${minutes !== 1 ? 's' : ''}`)

  return parts.join(', ') || 'Just started'
}

/**
 * Format file size
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`
}

/**
 * Truncate text with ellipsis
 */
export function truncate(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text
  return text.substring(0, maxLength - 3) + '...'
}

/**
 * Format player name for display
 */
export function formatPlayerName(name: string): string {
  return name
    .split(' ')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(' ')
}

/**
 * Get vocation abbreviation
 */
export function getVocationAbbr(vocation: string): string {
  const abbrs: Record<string, string> = {
    'None': 'N',
    'Knight': 'K',
    'Elite Knight': 'EK',
    'Paladin': 'P',
    'Royal Paladin': 'RP',
    'Sorcerer': 'S',
    'Master Sorcerer': 'MS',
    'Druid': 'D',
    'Elder Druid': 'ED',
  }
  return abbrs[vocation] || vocation.charAt(0)
}

/**
 * Get vocation color
 */
export function getVocationColor(vocation: string): string {
  const colors: Record<string, string> = {
    'None': 'text-gray-400',
    'Knight': 'text-red-400',
    'Elite Knight': 'text-red-500',
    'Paladin': 'text-yellow-400',
    'Royal Paladin': 'text-yellow-500',
    'Sorcerer': 'text-blue-400',
    'Master Sorcerer': 'text-blue-500',
    'Druid': 'text-green-400',
    'Elder Druid': 'text-green-500',
  }
  return colors[vocation] || 'text-gray-400'
}

