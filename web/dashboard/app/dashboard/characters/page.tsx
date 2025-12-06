'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import Link from 'next/link'
import { 
  Plus, 
  Shield, 
  Wand2, 
  Target,
  MoreVertical,
  Edit,
  Trash2,
  ArrowRightLeft,
  Globe,
  Clock,
  TrendingUp,
  Sparkles,
  Crown,
  Swords
} from 'lucide-react'
import * as DropdownMenu from '@radix-ui/react-dropdown-menu'
import * as Dialog from '@radix-ui/react-dialog'

const realmIcons: Record<string, any> = {
  'Shadowveil': Sparkles,
  'Aetheria': Crown,
  'Warbound': Swords,
  'Mythara': Shield,
}

const realmColors: Record<string, string> = {
  'Shadowveil': 'from-purple-900 to-slate-900 border-purple-500/30',
  'Aetheria': 'from-blue-900 to-indigo-900 border-blue-500/30',
  'Warbound': 'from-red-900 to-orange-900 border-red-500/30',
  'Mythara': 'from-amber-900 to-yellow-900 border-amber-500/30',
}

const vocationIcons: Record<string, any> = {
  'Elite Knight': Shield,
  'Royal Paladin': Target,
  'Elder Druid': Wand2,
  'Master Sorcerer': Wand2,
}

const vocationColors: Record<string, string> = {
  'Elite Knight': 'text-red-400',
  'Royal Paladin': 'text-yellow-400',
  'Elder Druid': 'text-green-400',
  'Master Sorcerer': 'text-blue-400',
}

const characters = [
  { 
    id: 1,
    name: 'ShadowKnight', 
    level: 312, 
    vocation: 'Elite Knight',
    realm: 'Shadowveil',
    online: true,
    lastLogin: 'Now',
    created: '2024-06-15',
    playTime: '847 hours',
    experience: 1847293847,
    skills: { axe: 112, shielding: 98, magic: 12 },
    outfit: { head: 95, body: 118, legs: 114, feet: 118 },
  },
  { 
    id: 2,
    name: 'MysticDruid', 
    level: 245, 
    vocation: 'Elder Druid',
    realm: 'Aetheria',
    online: false,
    lastLogin: '2 hours ago',
    created: '2024-08-20',
    playTime: '523 hours',
    experience: 892847291,
    skills: { magic: 89, shielding: 45, distance: 23 },
    outfit: { head: 0, body: 132, legs: 132, feet: 132 },
  },
  { 
    id: 3,
    name: 'SwiftArrow', 
    level: 198, 
    vocation: 'Royal Paladin',
    realm: 'Warbound',
    online: false,
    lastLogin: '1 day ago',
    created: '2024-09-10',
    playTime: '298 hours',
    experience: 384728193,
    skills: { distance: 95, shielding: 78, magic: 34 },
    outfit: { head: 78, body: 88, legs: 88, feet: 88 },
  },
  { 
    id: 4,
    name: 'StormMage', 
    level: 92, 
    vocation: 'Master Sorcerer',
    realm: 'Mythara',
    online: false,
    lastLogin: '3 days ago',
    created: '2024-11-01',
    playTime: '87 hours',
    experience: 48291032,
    skills: { magic: 56, shielding: 32, distance: 10 },
    outfit: { head: 0, body: 144, legs: 144, feet: 144 },
  },
]

function formatNumber(num: number): string {
  if (num >= 1e9) return `${(num / 1e9).toFixed(2)}B`
  if (num >= 1e6) return `${(num / 1e6).toFixed(2)}M`
  if (num >= 1e3) return `${(num / 1e3).toFixed(2)}K`
  return num.toLocaleString()
}

export default function CharactersPage() {
  const [selectedCharacter, setSelectedCharacter] = useState<typeof characters[0] | null>(null)
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false)

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex items-center justify-between"
      >
        <div>
          <h1 className="text-2xl font-display font-bold mb-1">Characters</h1>
          <p className="text-shadow-400">Manage your characters across all realms</p>
        </div>
        <Link href="/dashboard/characters/create" className="btn-primary flex items-center space-x-2">
          <Plus className="w-5 h-5" />
          <span>Create Character</span>
        </Link>
      </motion.div>

      {/* Characters Grid */}
      <div className="grid md:grid-cols-2 gap-6">
        {characters.map((character, index) => {
          const VocIcon = vocationIcons[character.vocation] || Shield
          const RealmIcon = realmIcons[character.realm] || Globe
          const realmColor = realmColors[character.realm] || 'from-shadow-800 to-shadow-900'
          
          return (
            <motion.div
              key={character.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.1 }}
              className={`relative bg-gradient-to-br ${realmColor} rounded-2xl border overflow-hidden group`}
            >
              {/* Header */}
              <div className="p-5 border-b border-white/10">
                <div className="flex items-start justify-between">
                  <div className="flex items-center space-x-4">
                    <div className="w-14 h-14 bg-black/30 rounded-xl flex items-center justify-center">
                      <VocIcon className={`w-8 h-8 ${vocationColors[character.vocation]}`} />
                    </div>
                    <div>
                      <div className="flex items-center space-x-2 mb-1">
                        <h3 className="font-display font-bold text-xl text-white">{character.name}</h3>
                        {character.online && (
                          <span className="flex items-center space-x-1 px-2 py-0.5 bg-green-500/20 text-green-400 text-xs rounded-full">
                            <span className="w-1.5 h-1.5 bg-green-400 rounded-full animate-pulse" />
                            <span>Online</span>
                          </span>
                        )}
                      </div>
                      <p className="text-white/70 text-sm">
                        Level {character.level} {character.vocation}
                      </p>
                    </div>
                  </div>
                  <DropdownMenu.Root>
                    <DropdownMenu.Trigger asChild>
                      <button className="p-2 hover:bg-white/10 rounded-lg transition-colors">
                        <MoreVertical className="w-5 h-5 text-white/50" />
                      </button>
                    </DropdownMenu.Trigger>
                    <DropdownMenu.Portal>
                      <DropdownMenu.Content className="w-48 bg-shadow-900 border border-shadow-700 rounded-xl shadow-xl overflow-hidden z-50" sideOffset={5}>
                        <DropdownMenu.Item className="flex items-center space-x-2 px-4 py-2.5 text-shadow-300 hover:text-white hover:bg-shadow-800 cursor-pointer focus:outline-none">
                          <Edit className="w-4 h-4" />
                          <span>Edit Character</span>
                        </DropdownMenu.Item>
                        <DropdownMenu.Item className="flex items-center space-x-2 px-4 py-2.5 text-shadow-300 hover:text-white hover:bg-shadow-800 cursor-pointer focus:outline-none">
                          <ArrowRightLeft className="w-4 h-4" />
                          <span>Transfer Realm</span>
                        </DropdownMenu.Item>
                        <DropdownMenu.Separator className="h-px bg-shadow-700 my-1" />
                        <DropdownMenu.Item 
                          onClick={() => {
                            setSelectedCharacter(character)
                            setDeleteDialogOpen(true)
                          }}
                          className="flex items-center space-x-2 px-4 py-2.5 text-red-400 hover:text-red-300 hover:bg-shadow-800 cursor-pointer focus:outline-none"
                        >
                          <Trash2 className="w-4 h-4" />
                          <span>Delete Character</span>
                        </DropdownMenu.Item>
                      </DropdownMenu.Content>
                    </DropdownMenu.Portal>
                  </DropdownMenu.Root>
                </div>
              </div>

              {/* Stats */}
              <div className="p-5">
                <div className="grid grid-cols-2 gap-4 mb-4">
                  <div className="bg-black/20 rounded-lg p-3">
                    <div className="flex items-center space-x-2 text-white/50 text-xs mb-1">
                      <TrendingUp className="w-3.5 h-3.5" />
                      <span>Experience</span>
                    </div>
                    <div className="text-white font-bold">{formatNumber(character.experience)}</div>
                  </div>
                  <div className="bg-black/20 rounded-lg p-3">
                    <div className="flex items-center space-x-2 text-white/50 text-xs mb-1">
                      <Clock className="w-3.5 h-3.5" />
                      <span>Play Time</span>
                    </div>
                    <div className="text-white font-bold">{character.playTime}</div>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <RealmIcon className="w-4 h-4 text-white/50" />
                    <span className="text-white/70 text-sm">{character.realm}</span>
                  </div>
                  <span className="text-white/50 text-xs">
                    {character.online ? 'Playing now' : `Last online ${character.lastLogin}`}
                  </span>
                </div>
              </div>

              {/* Quick Actions */}
              <div className="px-5 pb-5 flex space-x-2">
                <Link href={`/dashboard/characters/${character.id}`} className="flex-1 btn-secondary text-sm py-2 text-center">
                  View Profile
                </Link>
                <button className="flex-1 btn-primary text-sm py-2">
                  Play Now
                </button>
              </div>
            </motion.div>
          )
        })}
      </div>

      {/* Create New Character Card */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.4 }}
      >
        <Link 
          href="/dashboard/characters/create"
          className="block p-8 bg-shadow-900/30 border-2 border-dashed border-shadow-700 rounded-2xl hover:border-accent-500/50 hover:bg-shadow-900/50 transition-all text-center group"
        >
          <div className="w-16 h-16 bg-shadow-800 rounded-full flex items-center justify-center mx-auto mb-4 group-hover:bg-accent-500/20 transition-colors">
            <Plus className="w-8 h-8 text-shadow-500 group-hover:text-accent-500 transition-colors" />
          </div>
          <h3 className="font-semibold text-white mb-2">Create New Character</h3>
          <p className="text-shadow-400 text-sm max-w-sm mx-auto">
            Start a new adventure in any realm. Choose your vocation and customize your appearance.
          </p>
        </Link>
      </motion.div>

      {/* Delete Dialog */}
      <Dialog.Root open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/60 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-md bg-shadow-900 border border-shadow-700 rounded-2xl p-6 z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-2">
              Delete Character
            </Dialog.Title>
            <Dialog.Description className="text-shadow-400 mb-6">
              Are you sure you want to delete <span className="text-white font-medium">{selectedCharacter?.name}</span>? 
              This action cannot be undone. The character will be recoverable for 30 days.
            </Dialog.Description>
            <div className="flex space-x-3">
              <Dialog.Close asChild>
                <button className="flex-1 btn-secondary">Cancel</button>
              </Dialog.Close>
              <button 
                onClick={() => setDeleteDialogOpen(false)}
                className="flex-1 bg-red-500 hover:bg-red-600 text-white font-semibold py-2.5 px-5 rounded-lg transition-all"
              >
                Delete Character
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}

