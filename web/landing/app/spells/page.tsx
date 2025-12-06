'use client'

import { useState, useMemo } from 'react'
import { motion } from 'framer-motion'
import { 
  Sparkles, Search, Flame, Snowflake, Zap, Leaf,
  Sun, Moon, Heart, Shield, Swords, Target,
  Book, Star, Gem, Loader2, AlertCircle
} from 'lucide-react'
import { useSpells } from '@/shared/hooks/useSpells'
import type { Spell, SpellElement, SpellType, Vocation } from '@/shared/types'

const elementIcons: Record<SpellElement, React.ComponentType<{ className?: string }>> = {
  fire: Flame,
  ice: Snowflake,
  energy: Zap,
  earth: Leaf,
  holy: Sun,
  death: Moon,
  physical: Swords,
  healing: Heart,
}

const elementColors: Record<SpellElement, string> = {
  fire: 'from-orange-500 to-red-500',
  ice: 'from-cyan-400 to-blue-500',
  energy: 'from-purple-400 to-violet-500',
  earth: 'from-green-500 to-emerald-600',
  holy: 'from-yellow-400 to-amber-500',
  death: 'from-gray-500 to-slate-700',
  physical: 'from-slate-400 to-slate-600',
  healing: 'from-green-400 to-teal-500',
}

const vocationColors: Record<string, string> = {
  'Knight': 'bg-red-500/20 text-red-400 border-red-500/30',
  'Elite Knight': 'bg-red-500/20 text-red-400 border-red-500/30',
  'Paladin': 'bg-amber-500/20 text-amber-400 border-amber-500/30',
  'Royal Paladin': 'bg-amber-500/20 text-amber-400 border-amber-500/30',
  'Sorcerer': 'bg-blue-500/20 text-blue-400 border-blue-500/30',
  'Master Sorcerer': 'bg-blue-500/20 text-blue-400 border-blue-500/30',
  'Druid': 'bg-green-500/20 text-green-400 border-green-500/30',
  'Elder Druid': 'bg-green-500/20 text-green-400 border-green-500/30',
  'None': 'bg-purple-500/20 text-purple-400 border-purple-500/30',
}

export default function SpellsPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedElement, setSelectedElement] = useState<SpellElement | 'all'>('all')
  const [selectedVocation, setSelectedVocation] = useState<Vocation | 'all'>('all')
  const [selectedType, setSelectedType] = useState<SpellType | 'all'>('all')
  const [sortBy, setSortBy] = useState<'name' | 'level' | 'mana' | 'cooldown'>('level')

  // Fetch spells from API
  const { data: spells, isLoading, error } = useSpells({
    element: selectedElement === 'all' ? undefined : selectedElement,
    type: selectedType === 'all' ? undefined : selectedType,
    vocation: selectedVocation === 'all' ? undefined : selectedVocation,
    search: searchQuery || undefined,
  })

  // Client-side filtering and sorting for additional flexibility
  const filteredSpells = useMemo(() => {
    if (!spells) return []
    
    return [...spells]
      .filter(spell => {
        const matchesSearch = !searchQuery || 
          spell.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
          spell.words.toLowerCase().includes(searchQuery.toLowerCase())
        return matchesSearch
      })
      .sort((a, b) => {
        if (sortBy === 'name') return a.name.localeCompare(b.name)
        if (sortBy === 'level') return a.level - b.level
        if (sortBy === 'mana') return a.mana - b.mana
        if (sortBy === 'cooldown') return a.cooldown - b.cooldown
        return 0
      })
  }, [spells, searchQuery, sortBy])

  return (
    <main className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950">
      {/* Hero */}
      <section className="relative py-20 overflow-hidden">
        <div className="absolute inset-0 bg-[url('/grid.svg')] opacity-5" />
        <div className="absolute inset-0 bg-gradient-to-b from-purple-500/5 via-transparent to-transparent" />
        
        <div className="container mx-auto px-4 relative z-10">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="text-center max-w-3xl mx-auto"
          >
            <div className="inline-flex items-center gap-2 px-4 py-2 bg-purple-500/10 border border-purple-500/30 rounded-full text-purple-400 text-sm mb-6">
              <Book className="w-4 h-4" />
              Spell Library
            </div>
            <h1 className="text-4xl md:text-5xl font-bold text-white mb-4">
              Master the <span className="text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-500">Arcane Arts</span>
            </h1>
            <p className="text-slate-400 text-lg">
              Explore over 600 spells across all vocations. Learn the incantations, requirements, and effects.
            </p>
          </motion.div>
        </div>
      </section>

      {/* Element Quick Filters */}
      <section className="container mx-auto px-4 -mt-8 mb-8">
        <div className="flex flex-wrap justify-center gap-3">
          <button
            onClick={() => setSelectedElement('all')}
            className={`flex items-center gap-2 px-4 py-2 rounded-xl border transition ${
              selectedElement === 'all'
                ? 'bg-white/10 border-white/30 text-white'
                : 'bg-slate-800/50 border-slate-700/50 text-slate-400 hover:border-slate-600'
            }`}
          >
            <Sparkles className="w-4 h-4" />
            <span className="text-sm font-medium">All Elements</span>
          </button>
          {(Object.keys(elementIcons) as SpellElement[]).map((element) => {
            const Icon = elementIcons[element]
            const isActive = selectedElement === element
            return (
              <button
                key={element}
                onClick={() => setSelectedElement(element)}
                className={`flex items-center gap-2 px-4 py-2 rounded-xl border transition ${
                  isActive
                    ? `bg-gradient-to-r ${elementColors[element]} text-white border-transparent`
                    : 'bg-slate-800/50 border-slate-700/50 text-slate-400 hover:border-slate-600'
                }`}
              >
                <Icon className="w-4 h-4" />
                <span className="capitalize text-sm font-medium">{element}</span>
              </button>
            )
          })}
        </div>
      </section>

      {/* Filters Bar */}
      <section className="container mx-auto px-4 mb-8">
        <div className="flex flex-wrap items-center gap-4 bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          {/* Search */}
          <div className="relative flex-1 min-w-[250px]">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
            <input
              type="text"
              placeholder="Search by name or incantation..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-slate-900/50 border border-slate-700 rounded-lg pl-10 pr-4 py-2 text-white text-sm placeholder-slate-500 focus:outline-none focus:border-purple-500/50"
            />
          </div>

          {/* Vocation Filter */}
          <select
            value={selectedVocation}
            onChange={(e) => setSelectedVocation(e.target.value as Vocation | 'all')}
            className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-purple-500/50"
          >
            <option value="all">All Vocations</option>
            <option value="Knight">Knight</option>
            <option value="Paladin">Paladin</option>
            <option value="Sorcerer">Sorcerer</option>
            <option value="Druid">Druid</option>
          </select>

          {/* Type Filter */}
          <select
            value={selectedType}
            onChange={(e) => setSelectedType(e.target.value as SpellType | 'all')}
            className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-purple-500/50"
          >
            <option value="all">All Types</option>
            <option value="attack">Attack</option>
            <option value="healing">Healing</option>
            <option value="support">Support</option>
            <option value="summon">Summon</option>
          </select>

          {/* Sort */}
          <select
            value={sortBy}
            onChange={(e) => setSortBy(e.target.value as typeof sortBy)}
            className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-purple-500/50"
          >
            <option value="level">Sort by Level</option>
            <option value="name">Sort by Name</option>
            <option value="mana">Sort by Mana</option>
            <option value="cooldown">Sort by Cooldown</option>
          </select>

          <div className="text-slate-500 text-sm">
            {isLoading ? 'Loading...' : `${filteredSpells.length} spells found`}
          </div>
        </div>
      </section>

      {/* Spells Grid */}
      <section className="container mx-auto px-4 pb-20">
        {isLoading ? (
          <div className="flex items-center justify-center py-20">
            <Loader2 className="w-12 h-12 animate-spin text-purple-400" />
          </div>
        ) : error ? (
          <div className="flex flex-col items-center justify-center py-20 text-red-400">
            <AlertCircle className="w-12 h-12 mb-4" />
            <p>Failed to load spells. Please try again.</p>
          </div>
        ) : filteredSpells.length === 0 ? (
          <div className="text-center py-20">
            <Sparkles className="w-12 h-12 text-slate-600 mx-auto mb-4" />
            <p className="text-slate-400">No spells found matching your criteria</p>
          </div>
        ) : (
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
            {filteredSpells.map((spell, idx) => {
              const ElementIcon = elementIcons[spell.element]
              return (
                <motion.div
                  key={spell.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: Math.min(idx * 0.02, 0.5) }}
                  className="group bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden hover:border-purple-500/50 transition-all hover:shadow-lg hover:shadow-purple-500/10"
                >
                  {/* Header */}
                  <div className={`p-4 bg-gradient-to-r ${elementColors[spell.element]} bg-opacity-10`}>
                    <div className="flex items-start justify-between">
                      <div className="flex items-center gap-3">
                        <div className={`w-10 h-10 rounded-lg bg-gradient-to-br ${elementColors[spell.element]} flex items-center justify-center`}>
                          <ElementIcon className="w-5 h-5 text-white" />
                        </div>
                        <div>
                          <h3 className="text-white font-bold">{spell.name}</h3>
                          <p className="text-slate-400 text-sm font-mono">"{spell.words}"</p>
                        </div>
                      </div>
                      <div className="flex flex-col items-end gap-1">
                        {spell.premium && (
                          <span className="px-2 py-0.5 bg-amber-500/20 text-amber-400 text-xs rounded flex items-center gap-1">
                            <Star className="w-3 h-3" />
                            Premium
                          </span>
                        )}
                        {spell.rune && (
                          <span className="px-2 py-0.5 bg-cyan-500/20 text-cyan-400 text-xs rounded flex items-center gap-1">
                            <Gem className="w-3 h-3" />
                            Rune
                          </span>
                        )}
                      </div>
                    </div>
                  </div>

                  {/* Stats */}
                  <div className="p-4 grid grid-cols-3 gap-4 border-b border-slate-700/50">
                    <div className="text-center">
                      <p className="text-purple-400 font-bold">{spell.level}</p>
                      <p className="text-slate-500 text-xs">Level</p>
                    </div>
                    <div className="text-center">
                      <p className="text-blue-400 font-bold">{spell.mana}</p>
                      <p className="text-slate-500 text-xs">Mana</p>
                    </div>
                    <div className="text-center">
                      <p className="text-amber-400 font-bold">{spell.cooldown}s</p>
                      <p className="text-slate-500 text-xs">Cooldown</p>
                    </div>
                  </div>

                  {/* Description */}
                  <div className="p-4">
                    <p className="text-slate-400 text-sm mb-3">{spell.description}</p>
                    
                    <div className="flex flex-wrap gap-2 mb-3">
                      {spell.damage && (
                        <span className="px-2 py-1 bg-red-500/20 text-red-400 text-xs rounded flex items-center gap-1">
                          <Swords className="w-3 h-3" />
                          {spell.damage.min}-{spell.damage.max} dmg
                        </span>
                      )}
                      {spell.healing && (
                        <span className="px-2 py-1 bg-green-500/20 text-green-400 text-xs rounded flex items-center gap-1">
                          <Heart className="w-3 h-3" />
                          {spell.healing.min}-{spell.healing.max} heal
                        </span>
                      )}
                      {spell.area && (
                        <span className="px-2 py-1 bg-purple-500/20 text-purple-400 text-xs rounded flex items-center gap-1">
                          <Target className="w-3 h-3" />
                          AoE
                        </span>
                      )}
                    </div>

                    {/* Vocations */}
                    <div className="flex flex-wrap gap-1">
                      {spell.vocation.map(voc => (
                        <span 
                          key={voc} 
                          className={`px-2 py-0.5 text-xs rounded border ${vocationColors[voc] || vocationColors['None']}`}
                        >
                          {voc}
                        </span>
                      ))}
                    </div>
                  </div>
                </motion.div>
              )
            })}
          </div>
        )}
      </section>
    </main>
  )
}
