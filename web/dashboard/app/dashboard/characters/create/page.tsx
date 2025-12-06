'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import Link from 'next/link'
import { ArrowLeft, Shield, Target, Wand2, Leaf, User, AlertCircle, CheckCircle } from 'lucide-react'

const realms = [
  { id: 'shadowveil', name: 'Shadowveil', description: 'Dark theme, 5x exp rate, open PvP', color: 'border-red-500 hover:bg-red-500/10', icon: '🌑' },
  { id: 'aetheria', name: 'Aetheria', description: 'Mythic theme, 3x exp rate, optional PvP', color: 'border-blue-500 hover:bg-blue-500/10', icon: '✨' },
  { id: 'warbound', name: 'Warbound', description: 'PvP focused, 10x exp rate, hardcore', color: 'border-orange-500 hover:bg-orange-500/10', icon: '⚔️' },
  { id: 'mythara', name: 'Mythara', description: 'Classic theme, 1x exp rate, retro', color: 'border-green-500 hover:bg-green-500/10', icon: '🏰' },
]

const vocations = [
  { id: 'none', name: 'No Vocation', description: 'Choose your path later (level 8)', icon: User, color: 'text-gray-400', stats: { hp: 5, mp: 5, cap: 10 } },
  { id: 'knight', name: 'Knight', description: 'Melee fighter with high defense', icon: Shield, color: 'text-red-400', stats: { hp: 15, mp: 5, cap: 25 } },
  { id: 'paladin', name: 'Paladin', description: 'Ranged fighter with holy magic', icon: Target, color: 'text-yellow-400', stats: { hp: 10, mp: 15, cap: 20 } },
  { id: 'sorcerer', name: 'Sorcerer', description: 'Powerful offensive magic', icon: Wand2, color: 'text-blue-400', stats: { hp: 5, mp: 30, cap: 10 } },
  { id: 'druid', name: 'Druid', description: 'Healing and nature magic', icon: Leaf, color: 'text-green-400', stats: { hp: 5, mp: 30, cap: 10 } },
]

export default function CreateCharacterPage() {
  const [name, setName] = useState('')
  const [realm, setRealm] = useState('')
  const [vocation, setVocation] = useState('none')
  const [sex, setSex] = useState<'male' | 'female'>('male')
  const [nameError, setNameError] = useState('')
  const [isChecking, setIsChecking] = useState(false)
  const [isAvailable, setIsAvailable] = useState<boolean | null>(null)
  const [isCreating, setIsCreating] = useState(false)

  const validateName = (value: string) => {
    if (value.length < 3) return 'Name must be at least 3 characters'
    if (value.length > 30) return 'Name cannot exceed 30 characters'
    if (!/^[A-Za-z][A-Za-z ']{1,28}[A-Za-z]$/.test(value)) {
      return 'Name must start/end with letter, contain only letters, spaces, apostrophes'
    }
    return ''
  }

  const checkNameAvailability = async () => {
    const error = validateName(name)
    if (error) {
      setNameError(error)
      return
    }
    setIsChecking(true)
    setNameError('')
    // Simulate API check
    setTimeout(() => {
      setIsAvailable(name.toLowerCase() !== 'admin')
      setIsChecking(false)
    }, 500)
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!name || !realm || !isAvailable) return
    
    setIsCreating(true)
    // Simulate API call
    setTimeout(() => {
      window.location.href = '/dashboard/characters'
    }, 1000)
  }

  const selectedVocation = vocations.find(v => v.id === vocation)

  return (
    <div className="max-w-4xl mx-auto">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <Link href="/dashboard/characters" className="inline-flex items-center gap-2 text-shadow-400 hover:text-white mb-6 transition-colors">
          <ArrowLeft className="w-4 h-4" />
          Back to Characters
        </Link>
        <h1 className="text-2xl font-display font-bold mb-2">Create New Character</h1>
        <p className="text-shadow-400 mb-8">Begin a new adventure in Shadow OT</p>

        <form onSubmit={handleSubmit} className="space-y-8">
          {/* Character Name */}
          <div className="card">
            <h2 className="text-lg font-semibold text-white mb-4">Character Name</h2>
            <div className="flex gap-4">
              <div className="flex-1">
                <input
                  type="text"
                  value={name}
                  onChange={(e) => {
                    setName(e.target.value)
                    setIsAvailable(null)
                    setNameError('')
                  }}
                  placeholder="Enter character name..."
                  className="w-full px-4 py-3 bg-shadow-800 border border-shadow-600 rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50"
                />
                {nameError && (
                  <p className="mt-2 text-sm text-red-400 flex items-center gap-1">
                    <AlertCircle className="w-4 h-4" />
                    {nameError}
                  </p>
                )}
                {isAvailable === true && (
                  <p className="mt-2 text-sm text-green-400 flex items-center gap-1">
                    <CheckCircle className="w-4 h-4" />
                    Name is available!
                  </p>
                )}
                {isAvailable === false && (
                  <p className="mt-2 text-sm text-red-400 flex items-center gap-1">
                    <AlertCircle className="w-4 h-4" />
                    Name is already taken
                  </p>
                )}
              </div>
              <button
                type="button"
                onClick={checkNameAvailability}
                disabled={!name || isChecking}
                className="btn-secondary px-6 disabled:opacity-50"
              >
                {isChecking ? 'Checking...' : 'Check'}
              </button>
            </div>
          </div>

          {/* Select Realm */}
          <div className="card">
            <h2 className="text-lg font-semibold text-white mb-4">Select Realm</h2>
            <div className="grid sm:grid-cols-2 gap-4">
              {realms.map((r) => (
                <button
                  key={r.id}
                  type="button"
                  onClick={() => setRealm(r.id)}
                  className={`p-4 rounded-xl border-2 text-left transition-all ${
                    realm === r.id
                      ? `${r.color} bg-opacity-20`
                      : 'border-shadow-700 hover:border-shadow-500'
                  }`}
                >
                  <div className="flex items-center gap-3 mb-2">
                    <span className="text-2xl">{r.icon}</span>
                    <span className="font-semibold text-white">{r.name}</span>
                  </div>
                  <p className="text-shadow-400 text-sm">{r.description}</p>
                </button>
              ))}
            </div>
          </div>

          {/* Select Vocation */}
          <div className="card">
            <h2 className="text-lg font-semibold text-white mb-4">Select Vocation</h2>
            <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-4">
              {vocations.map((v) => (
                <button
                  key={v.id}
                  type="button"
                  onClick={() => setVocation(v.id)}
                  className={`p-4 rounded-xl border-2 text-left transition-all ${
                    vocation === v.id
                      ? 'border-accent-500 bg-accent-500/10'
                      : 'border-shadow-700 hover:border-shadow-500'
                  }`}
                >
                  <div className="flex items-center gap-3 mb-2">
                    <v.icon className={`w-6 h-6 ${v.color}`} />
                    <span className="font-semibold text-white">{v.name}</span>
                  </div>
                  <p className="text-shadow-400 text-sm mb-3">{v.description}</p>
                  <div className="flex gap-4 text-xs">
                    <span className="text-red-400">HP +{v.stats.hp}/lvl</span>
                    <span className="text-blue-400">MP +{v.stats.mp}/lvl</span>
                    <span className="text-yellow-400">Cap +{v.stats.cap}/lvl</span>
                  </div>
                </button>
              ))}
            </div>
          </div>

          {/* Select Sex */}
          <div className="card">
            <h2 className="text-lg font-semibold text-white mb-4">Select Sex</h2>
            <div className="flex gap-4">
              <button
                type="button"
                onClick={() => setSex('male')}
                className={`flex-1 p-4 rounded-xl border-2 transition-all ${
                  sex === 'male'
                    ? 'border-accent-500 bg-accent-500/10'
                    : 'border-shadow-700 hover:border-shadow-500'
                }`}
              >
                <span className="text-2xl mb-2 block">👨</span>
                <span className="font-semibold text-white">Male</span>
              </button>
              <button
                type="button"
                onClick={() => setSex('female')}
                className={`flex-1 p-4 rounded-xl border-2 transition-all ${
                  sex === 'female'
                    ? 'border-accent-500 bg-accent-500/10'
                    : 'border-shadow-700 hover:border-shadow-500'
                }`}
              >
                <span className="text-2xl mb-2 block">👩</span>
                <span className="font-semibold text-white">Female</span>
              </button>
            </div>
          </div>

          {/* Summary */}
          {name && realm && (
            <div className="card bg-gradient-to-br from-accent-500/10 to-purple-500/10 border-accent-500/30">
              <h2 className="text-lg font-semibold text-white mb-4">Character Summary</h2>
              <div className="grid sm:grid-cols-2 gap-4 text-sm">
                <div>
                  <span className="text-shadow-400">Name:</span>
                  <span className="ml-2 text-white font-medium">{name}</span>
                </div>
                <div>
                  <span className="text-shadow-400">Realm:</span>
                  <span className="ml-2 text-white font-medium capitalize">{realm}</span>
                </div>
                <div>
                  <span className="text-shadow-400">Vocation:</span>
                  <span className="ml-2 text-white font-medium">{selectedVocation?.name}</span>
                </div>
                <div>
                  <span className="text-shadow-400">Sex:</span>
                  <span className="ml-2 text-white font-medium capitalize">{sex}</span>
                </div>
              </div>
            </div>
          )}

          {/* Submit */}
          <div className="flex justify-end gap-4">
            <Link href="/dashboard/characters" className="btn-secondary">
              Cancel
            </Link>
            <button
              type="submit"
              disabled={!name || !realm || isAvailable !== true || isCreating}
              className="btn-primary px-8 disabled:opacity-50"
            >
              {isCreating ? 'Creating...' : 'Create Character'}
            </button>
          </div>
        </form>
      </motion.div>
    </div>
  )
}

