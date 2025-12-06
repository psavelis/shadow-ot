import React from 'react'
import { cn } from '../../utils/cn'
import { Character } from '../../types'
import { VocationIcon } from './VocationIcon'
import { RealmBadge } from './RealmBadge'
import { OnlineIndicator } from './OnlineIndicator'
import { formatRelativeTime } from '../../utils/format'

interface CharacterCardProps {
  character: Character
  onClick?: () => void
  selected?: boolean
  showRealm?: boolean
  className?: string
}

export function CharacterCard({ character, onClick, selected, showRealm = true, className }: CharacterCardProps) {
  return (
    <div
      onClick={onClick}
      className={cn(
        'p-4 rounded-xl border transition-all',
        'bg-shadow-800/50 border-shadow-700',
        onClick && 'cursor-pointer hover:border-accent-500/50 hover:bg-shadow-800',
        selected && 'border-accent-500 ring-2 ring-accent-500/30',
        className
      )}
    >
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-3">
          <div className="w-12 h-12 rounded-lg bg-shadow-700 flex items-center justify-center">
            <VocationIcon vocation={character.vocation} size="lg" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <span className="font-semibold text-white">{character.name}</span>
              <OnlineIndicator online={character.online} />
            </div>
            <p className="text-sm text-shadow-400">Level {character.level} {character.vocation}</p>
          </div>
        </div>
        {showRealm && <RealmBadge realm={character.realm} size="sm" />}
      </div>
      <div className="flex items-center justify-between text-sm">
        <span className="text-shadow-500">Last login: {character.online ? 'Now' : formatRelativeTime(character.lastLogin)}</span>
        <span className="text-shadow-400">{character.experience.toLocaleString()} exp</span>
      </div>
    </div>
  )
}

interface CharacterCardSkeletonProps {
  className?: string
}

export function CharacterCardSkeleton({ className }: CharacterCardSkeletonProps) {
  return (
    <div className={cn('p-4 rounded-xl border bg-shadow-800/50 border-shadow-700 animate-pulse', className)}>
      <div className="flex items-center gap-3 mb-3">
        <div className="w-12 h-12 rounded-lg bg-shadow-700" />
        <div className="space-y-2">
          <div className="h-4 w-24 bg-shadow-700 rounded" />
          <div className="h-3 w-32 bg-shadow-700 rounded" />
        </div>
      </div>
      <div className="flex justify-between">
        <div className="h-3 w-24 bg-shadow-700 rounded" />
        <div className="h-3 w-16 bg-shadow-700 rounded" />
      </div>
    </div>
  )
}

