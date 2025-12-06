'use client'

import { useState } from 'react'
import { useParams, useRouter } from 'next/navigation'
import Link from 'next/link'
import { motion } from 'framer-motion'
import {
  ArrowLeft, Shield, Sword, Heart, Zap, Star, Trophy, Skull,
  Clock, MapPin, Users, Crown, ChevronRight, AlertTriangle,
  ArrowRightLeft, Trash2, Edit3, History, Award, Target,
  Loader2
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'
import * as Tabs from '@radix-ui/react-tabs'
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer, AreaChart, Area } from 'recharts'
import { 
  useCharacter, 
  useCharacterDeaths, 
  useCharacterAchievements,
  useDeleteCharacter,
  useTransferCharacter 
} from '@/shared/hooks/useCharacters'
import { useRealms } from '@/shared/hooks/useRealms'
import { getOutfitSprite, getRealmColors } from '@/shared/utils/assets'
import type { RealmId } from '@/shared/types'

export default function CharacterDetailPage() {
  const params = useParams()
  const router = useRouter()
  const characterId = params.id as string
  
  const [showTransferDialog, setShowTransferDialog] = useState(false)
  const [showDeleteDialog, setShowDeleteDialog] = useState(false)
  const [selectedRealm, setSelectedRealm] = useState<RealmId | ''>('')
  const [deleteConfirm, setDeleteConfirm] = useState('')

  // Fetch character data from API
  const { data: character, isLoading, error } = useCharacter(characterId)
  const { data: deaths } = useCharacterDeaths(characterId)
  const { data: achievements } = useCharacterAchievements(characterId)
  const { data: realms } = useRealms()
  
  // Mutations
  const deleteCharacter = useDeleteCharacter()
  const transferCharacter = useTransferCharacter()

  const formatNumber = (n: number) => n?.toLocaleString() || '0'
  const formatGold = (n: number) => {
    if (!n) return '0'
    if (n >= 1000000000) return `${(n / 1000000000).toFixed(2)}kkk`
    if (n >= 1000000) return `${(n / 1000000).toFixed(2)}kk`
    if (n >= 1000) return `${(n / 1000).toFixed(1)}k`
    return n.toString()
  }

  const handleTransfer = async () => {
    if (!selectedRealm || !character) return
    try {
      await transferCharacter.mutateAsync({ id: characterId, targetRealm: selectedRealm as RealmId })
      setShowTransferDialog(false)
      router.refresh()
    } catch (err) {
      // Error handled by hook
    }
  }

  const handleDelete = async () => {
    if (deleteConfirm !== character?.name) return
    try {
      await deleteCharacter.mutateAsync(characterId)
      router.push('/dashboard/characters')
    } catch (err) {
      // Error handled by hook
    }
  }

  // Loading state
  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <Loader2 className="w-12 h-12 animate-spin text-amber-400" />
      </div>
    )
  }

  // Error state
  if (error || !character) {
    return (
      <div className="flex flex-col items-center justify-center min-h-[50vh] text-red-400">
        <AlertTriangle className="w-12 h-12 mb-4" />
        <h2 className="text-xl font-bold text-white mb-2">Character Not Found</h2>
        <p className="text-slate-400 mb-4">The character you're looking for doesn't exist or you don't have access.</p>
        <Link href="/dashboard/characters" className="text-amber-400 hover:text-amber-300 flex items-center gap-2">
          <ArrowLeft className="w-4 h-4" />
          Back to Characters
        </Link>
      </div>
    )
  }

  const realmColors = getRealmColors(character.realm)

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex flex-col sm:flex-row sm:items-center justify-between gap-4"
      >
        <div className="flex items-center gap-4">
          <Link
            href="/dashboard/characters"
            className="p-2 bg-slate-800/50 rounded-lg hover:bg-slate-700/50 transition"
          >
            <ArrowLeft className="w-5 h-5 text-slate-400" />
          </Link>
          <div className="flex items-center gap-4">
            <div className="w-16 h-16 bg-slate-800/50 rounded-xl border border-slate-700/50 overflow-hidden flex items-center justify-center">
              {character.outfit ? (
                <img 
                  src={getOutfitSprite(character.outfit)} 
                  alt={character.name}
                  className="w-12 h-12 object-contain"
                />
              ) : (
                <Shield className="w-8 h-8 text-slate-400" />
              )}
            </div>
            <div>
              <div className="flex items-center gap-3">
                <h1 className="text-2xl font-bold text-white">{character.name}</h1>
                {character.online && (
                  <span className="flex items-center gap-1 px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs rounded-full">
                    <span className="w-1.5 h-1.5 bg-emerald-400 rounded-full animate-pulse" />
                    Online
                  </span>
                )}
              </div>
              <p className="text-slate-400">
                Level {character.level} - {character.vocation}
              </p>
            </div>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={() => setShowTransferDialog(true)}
            className="flex items-center gap-2 px-4 py-2 bg-slate-800/50 border border-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-700/50 transition"
          >
            <ArrowRightLeft className="w-4 h-4" />
            Transfer
          </button>
          <button
            onClick={() => setShowDeleteDialog(true)}
            className="flex items-center gap-2 px-4 py-2 bg-red-500/20 border border-red-500/30 text-red-400 rounded-lg hover:bg-red-500/30 transition"
          >
            <Trash2 className="w-4 h-4" />
            Delete
          </button>
        </div>
      </motion.div>

      {/* Main Content */}
      <div className="grid lg:grid-cols-3 gap-6">
        {/* Stats Column */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="lg:col-span-2 space-y-6"
        >
          {/* Stats Card */}
          <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
            <h2 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
              <Zap className="w-5 h-5 text-amber-400" />
              Character Stats
            </h2>
            <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
              <div className="bg-slate-900/50 rounded-lg p-4">
                <div className="flex items-center gap-2 text-red-400 mb-2">
                  <Heart className="w-4 h-4" />
                  <span className="text-sm">Health</span>
                </div>
                <p className="text-white font-bold text-xl">{formatNumber(character.health)}</p>
                <p className="text-slate-500 text-xs">/ {formatNumber(character.healthMax)}</p>
              </div>
              <div className="bg-slate-900/50 rounded-lg p-4">
                <div className="flex items-center gap-2 text-blue-400 mb-2">
                  <Zap className="w-4 h-4" />
                  <span className="text-sm">Mana</span>
                </div>
                <p className="text-white font-bold text-xl">{formatNumber(character.mana)}</p>
                <p className="text-slate-500 text-xs">/ {formatNumber(character.manaMax)}</p>
              </div>
              <div className="bg-slate-900/50 rounded-lg p-4">
                <div className="flex items-center gap-2 text-amber-400 mb-2">
                  <Star className="w-4 h-4" />
                  <span className="text-sm">Experience</span>
                </div>
                <p className="text-white font-bold text-xl">{formatNumber(character.experience)}</p>
              </div>
              <div className="bg-slate-900/50 rounded-lg p-4">
                <div className="flex items-center gap-2 text-purple-400 mb-2">
                  <Trophy className="w-4 h-4" />
                  <span className="text-sm">Achievements</span>
                </div>
                <p className="text-white font-bold text-xl">{character.achievementPoints || character.achievements || 0}</p>
              </div>
            </div>
          </div>

          {/* Skills */}
          {character.skills && (
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <h2 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
                <Sword className="w-5 h-5 text-red-400" />
                Skills
              </h2>
              <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
                {Object.entries(character.skills).map(([skill, data]) => (
                  <div key={skill} className="bg-slate-900/50 rounded-lg p-3">
                    <div className="flex justify-between items-center mb-2">
                      <span className="text-slate-400 text-sm capitalize">
                        {skill.replace(/([A-Z])/g, ' $1').trim()}
                      </span>
                      <span className="text-white font-bold">{data.level}</span>
                    </div>
                    <div className="h-2 bg-slate-700 rounded-full overflow-hidden">
                      <div 
                        className="h-full bg-gradient-to-r from-amber-500 to-yellow-500 rounded-full"
                        style={{ width: `${data.percent}%` }}
                      />
                    </div>
                    <p className="text-slate-500 text-xs mt-1 text-right">{data.percent}%</p>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Deaths */}
          {deaths && deaths.length > 0 && (
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <h2 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
                <Skull className="w-5 h-5 text-red-400" />
                Recent Deaths
              </h2>
              <div className="space-y-3">
                {deaths.slice(0, 5).map((death) => (
                  <div key={death.id} className="flex items-center justify-between p-3 bg-slate-900/50 rounded-lg">
                    <div>
                      <p className="text-white">
                        Killed by <span className="text-red-400 font-medium">{death.killerName}</span>
                      </p>
                      <p className="text-slate-500 text-sm">at level {death.levelAtDeath}</p>
                    </div>
                    <div className="text-right">
                      <p className="text-slate-400 text-sm">
                        {new Date(death.timestamp).toLocaleDateString()}
                      </p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </motion.div>

        {/* Sidebar */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="space-y-6"
        >
          {/* Info Card */}
          <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
            <h2 className="text-lg font-bold text-white mb-4">Information</h2>
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-slate-400">Realm</span>
                <span 
                  className="font-medium capitalize"
                  style={{ color: realmColors.primary }}
                >
                  {character.realm}
                </span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-slate-400">Vocation</span>
                <span className="text-white">{character.vocation}</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-slate-400">Sex</span>
                <span className="text-white capitalize">{character.sex}</span>
              </div>
              {character.residence && (
                <div className="flex items-center justify-between">
                  <span className="text-slate-400">Residence</span>
                  <span className="text-white">{character.residence}</span>
                </div>
              )}
              {character.guild && (
                <div className="flex items-center justify-between">
                  <span className="text-slate-400">Guild</span>
                  <span className="text-white">{character.guild.name}</span>
                </div>
              )}
              <div className="flex items-center justify-between">
                <span className="text-slate-400">Last Login</span>
                <span className="text-white">
                  {character.lastLogin ? new Date(character.lastLogin).toLocaleDateString() : 'Unknown'}
                </span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-slate-400">Created</span>
                <span className="text-white">
                  {new Date(character.createdAt).toLocaleDateString()}
                </span>
              </div>
            </div>
          </div>

          {/* Balance Card */}
          {(character.balance !== undefined || character.bankBalance !== undefined) && (
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <h2 className="text-lg font-bold text-white mb-4">Balance</h2>
              <div className="space-y-4">
                {character.balance !== undefined && (
                  <div className="flex items-center justify-between">
                    <span className="text-slate-400">Gold</span>
                    <span className="text-amber-400 font-bold">{formatGold(character.balance)}</span>
                  </div>
                )}
                {character.bankBalance !== undefined && (
                  <div className="flex items-center justify-between">
                    <span className="text-slate-400">Bank</span>
                    <span className="text-amber-400 font-bold">{formatGold(character.bankBalance)}</span>
                  </div>
                )}
              </div>
            </div>
          )}
        </motion.div>
      </div>

      {/* Transfer Dialog */}
      <Dialog.Root open={showTransferDialog} onOpenChange={setShowTransferDialog}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-md bg-slate-900 border border-slate-700 rounded-xl p-6 z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-4">
              Transfer Character
            </Dialog.Title>
            <p className="text-slate-400 mb-6">
              Select a realm to transfer <span className="text-white font-medium">{character.name}</span> to.
            </p>
            <div className="space-y-3 mb-6">
              {realms?.filter(r => r.id !== character.realm && r.status === 'online').map((realm) => (
                <button
                  key={realm.id}
                  onClick={() => setSelectedRealm(realm.id)}
                  className={`w-full flex items-center justify-between p-4 rounded-lg border transition ${
                    selectedRealm === realm.id
                      ? 'bg-amber-500/20 border-amber-500/50'
                      : 'bg-slate-800/50 border-slate-700/50 hover:border-slate-600'
                  }`}
                >
                  <div>
                    <p className="text-white font-medium">{realm.name}</p>
                    <p className="text-slate-400 text-sm">{realm.pvpType}</p>
                  </div>
                  {selectedRealm === realm.id && (
                    <div className="w-5 h-5 rounded-full bg-amber-500 flex items-center justify-center">
                      <div className="w-2 h-2 bg-white rounded-full" />
                    </div>
                  )}
                </button>
              ))}
            </div>
            <div className="flex gap-3">
              <Dialog.Close asChild>
                <button className="flex-1 px-4 py-2 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-600/50 transition">
                  Cancel
                </button>
              </Dialog.Close>
              <button
                onClick={handleTransfer}
                disabled={!selectedRealm || transferCharacter.isPending}
                className="flex-1 px-4 py-2 bg-amber-500 text-white rounded-lg hover:bg-amber-600 transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
              >
                {transferCharacter.isPending ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    Transferring...
                  </>
                ) : (
                  'Transfer'
                )}
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>

      {/* Delete Dialog */}
      <Dialog.Root open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-md bg-slate-900 border border-slate-700 rounded-xl p-6 z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-4 flex items-center gap-2">
              <AlertTriangle className="w-6 h-6 text-red-400" />
              Delete Character
            </Dialog.Title>
            <p className="text-slate-400 mb-4">
              This action cannot be undone. All character data, items, and progress will be permanently deleted.
            </p>
            <p className="text-slate-300 mb-4">
              Type <span className="text-red-400 font-bold">{character.name}</span> to confirm:
            </p>
            <input
              type="text"
              value={deleteConfirm}
              onChange={(e) => setDeleteConfirm(e.target.value)}
              placeholder="Enter character name"
              className="w-full px-4 py-3 bg-slate-800/50 border border-slate-700 rounded-lg text-white placeholder-slate-500 focus:outline-none focus:border-red-500/50 mb-6"
            />
            <div className="flex gap-3">
              <Dialog.Close asChild>
                <button className="flex-1 px-4 py-2 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-600/50 transition">
                  Cancel
                </button>
              </Dialog.Close>
              <button
                onClick={handleDelete}
                disabled={deleteConfirm !== character.name || deleteCharacter.isPending}
                className="flex-1 px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
              >
                {deleteCharacter.isPending ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    Deleting...
                  </>
                ) : (
                  'Delete Forever'
                )}
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}
