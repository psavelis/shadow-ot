'use client'

import { useState, useMemo } from 'react'
import { motion } from 'framer-motion'
import { 
  Sparkles, Search, Filter, Flame, Snowflake, Zap, Leaf,
  Sun, Moon, Heart, Shield, Swords, Wind, Target, Eye,
  Book, ChevronDown, Star, Clock, Gem
} from 'lucide-react'

type SpellElement = 'fire' | 'ice' | 'energy' | 'earth' | 'holy' | 'death' | 'physical' | 'healing'
type SpellType = 'attack' | 'healing' | 'support' | 'summon'
type Vocation = 'knight' | 'paladin' | 'sorcerer' | 'druid' | 'all'

interface Spell {
  id: string
  name: string
  words: string
  element: SpellElement
  type: SpellType
  vocation: Vocation[]
  level: number
  mana: number
  cooldown: number
  premium: boolean
  description: string
  damage?: string
  healing?: string
  area?: boolean
  rune?: boolean
}

const spells: Spell[] = [
  // Sorcerer Attack Spells
  { id: '1', name: 'Exori Gran Flam', words: 'exori gran flam', element: 'fire', type: 'attack', vocation: ['sorcerer'], level: 35, mana: 530, cooldown: 4, premium: true, description: 'Shoots a powerful fire beam dealing massive fire damage.', damage: '400-700', area: false },
  { id: '2', name: 'Hell\'s Core', words: 'exevo gran mas flam', element: 'fire', type: 'attack', vocation: ['sorcerer'], level: 60, mana: 1100, cooldown: 40, premium: true, description: 'Creates a devastating explosion of hellfire around you.', damage: '350-600', area: true },
  { id: '3', name: 'Rage of the Skies', words: 'exevo gran mas vis', element: 'energy', type: 'attack', vocation: ['sorcerer'], level: 55, mana: 1000, cooldown: 40, premium: true, description: 'Calls down lightning bolts in a large area.', damage: '320-580', area: true },
  { id: '4', name: 'Great Energy Beam', words: 'exevo gran vis lux', element: 'energy', type: 'attack', vocation: ['sorcerer'], level: 29, mana: 340, cooldown: 4, premium: false, description: 'Fires a long beam of pure energy.', damage: '200-380', area: false },
  { id: '5', name: 'Energy Wave', words: 'exevo vis hur', element: 'energy', type: 'attack', vocation: ['sorcerer'], level: 38, mana: 560, cooldown: 8, premium: false, description: 'Sends a wave of energy in front of you.', damage: '250-450', area: true },
  
  // Druid Attack Spells
  { id: '6', name: 'Terra Wave', words: 'exevo tera hur', element: 'earth', type: 'attack', vocation: ['druid'], level: 38, mana: 560, cooldown: 8, premium: false, description: 'Unleashes a wave of earth energy.', damage: '250-450', area: true },
  { id: '7', name: 'Wrath of Nature', words: 'exevo gran mas tera', element: 'earth', type: 'attack', vocation: ['druid'], level: 55, mana: 1000, cooldown: 40, premium: true, description: 'Summons the wrath of nature in a large area.', damage: '320-580', area: true },
  { id: '8', name: 'Eternal Winter', words: 'exevo gran mas frigo', element: 'ice', type: 'attack', vocation: ['druid'], level: 60, mana: 1100, cooldown: 40, premium: true, description: 'Creates a blizzard of eternal winter.', damage: '350-600', area: true },
  { id: '9', name: 'Strong Ice Wave', words: 'exevo gran frigo hur', element: 'ice', type: 'attack', vocation: ['druid'], level: 40, mana: 600, cooldown: 8, premium: true, description: 'Sends a powerful ice wave forward.', damage: '280-500', area: true },
  
  // Healing Spells
  { id: '10', name: 'Exura Gran', words: 'exura gran', element: 'healing', type: 'healing', vocation: ['druid', 'sorcerer', 'paladin'], level: 11, mana: 70, cooldown: 1, premium: false, description: 'Heals moderate wounds.', healing: '100-180' },
  { id: '11', name: 'Exura Vita', words: 'exura vita', element: 'healing', type: 'healing', vocation: ['druid', 'sorcerer', 'paladin'], level: 70, mana: 160, cooldown: 1, premium: true, description: 'Heals severe wounds.', healing: '350-500' },
  { id: '12', name: 'Mass Healing', words: 'exura gran mas res', element: 'healing', type: 'healing', vocation: ['druid'], level: 36, mana: 150, cooldown: 2, premium: true, description: 'Heals all party members in range.', healing: '200-350', area: true },
  { id: '13', name: 'Ultimate Healing', words: 'exura san', element: 'healing', type: 'healing', vocation: ['druid', 'paladin'], level: 30, mana: 100, cooldown: 1, premium: false, description: 'Powerful single target heal.', healing: '250-400' },
  
  // Knight Spells
  { id: '14', name: 'Fierce Berserk', words: 'exori gran', element: 'physical', type: 'attack', vocation: ['knight'], level: 90, mana: 340, cooldown: 6, premium: true, description: 'Devastating melee attack hitting all nearby enemies.', damage: '300-550', area: true },
  { id: '15', name: 'Annihilation', words: 'exori gran ico', element: 'physical', type: 'attack', vocation: ['knight'], level: 110, mana: 300, cooldown: 30, premium: true, description: 'The ultimate knight attack.', damage: '500-900', area: false },
  { id: '16', name: 'Groundshaker', words: 'exori mas', element: 'physical', type: 'attack', vocation: ['knight'], level: 33, mana: 160, cooldown: 8, premium: true, description: 'Shakes the ground dealing damage to all nearby enemies.', damage: '150-280', area: true },
  
  // Paladin Spells
  { id: '17', name: 'Divine Missile', words: 'exori san', element: 'holy', type: 'attack', vocation: ['paladin'], level: 40, mana: 50, cooldown: 2, premium: true, description: 'Shoots a bolt of holy energy.', damage: '100-200', area: false },
  { id: '18', name: 'Ethereal Spear', words: 'exori con', element: 'physical', type: 'attack', vocation: ['paladin'], level: 23, mana: 25, cooldown: 2, premium: false, description: 'Throws an ethereal spear at the target.', damage: '50-120', area: false },
  { id: '19', name: 'Strong Ethereal Spear', words: 'exori gran con', element: 'physical', type: 'attack', vocation: ['paladin'], level: 90, mana: 55, cooldown: 8, premium: true, description: 'Throws a powerful ethereal spear.', damage: '150-300', area: false },
  
  // Support Spells
  { id: '20', name: 'Haste', words: 'utani hur', element: 'physical', type: 'support', vocation: ['all'], level: 14, mana: 60, cooldown: 2, premium: true, description: 'Increases your movement speed.', area: false },
  { id: '21', name: 'Strong Haste', words: 'utani gran hur', element: 'physical', type: 'support', vocation: ['sorcerer', 'druid'], level: 20, mana: 100, cooldown: 2, premium: true, description: 'Greatly increases movement speed.', area: false },
  { id: '22', name: 'Invisible', words: 'utana vid', element: 'physical', type: 'support', vocation: ['sorcerer', 'druid'], level: 35, mana: 440, cooldown: 2, premium: false, description: 'Makes you invisible to most creatures.', area: false },
  { id: '23', name: 'Magic Shield', words: 'utamo vita', element: 'physical', type: 'support', vocation: ['sorcerer', 'druid'], level: 14, mana: 50, cooldown: 2, premium: false, description: 'Converts damage to mana drain.', area: false },
  { id: '24', name: 'Protector', words: 'utamo tempo', element: 'physical', type: 'support', vocation: ['knight'], level: 55, mana: 200, cooldown: 2, premium: true, description: 'Greatly reduces incoming damage.', area: false },
  
  // Rune Spells
  { id: '25', name: 'Sudden Death', words: 'adori gran mort', element: 'death', type: 'attack', vocation: ['sorcerer', 'druid'], level: 45, mana: 985, cooldown: 2, premium: true, description: 'Creates the deadly Sudden Death rune.', damage: '350-600', rune: true },
  { id: '26', name: 'Great Fireball', words: 'adori mas flam', element: 'fire', type: 'attack', vocation: ['sorcerer', 'druid'], level: 30, mana: 530, cooldown: 2, premium: false, description: 'Creates the Great Fireball rune.', damage: '150-300', rune: true, area: true },
  { id: '27', name: 'Explosion', words: 'adevo mas hur', element: 'physical', type: 'attack', vocation: ['sorcerer', 'druid'], level: 31, mana: 570, cooldown: 2, premium: false, description: 'Creates the Explosion rune.', damage: '180-350', rune: true, area: true },
  { id: '28', name: 'Ultimate Healing Rune', words: 'adura vita', element: 'healing', type: 'healing', vocation: ['druid'], level: 24, mana: 400, cooldown: 2, premium: false, description: 'Creates the Ultimate Healing rune.', healing: '250-400', rune: true },
]

const elementIcons: Record<SpellElement, any> = {
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
  knight: 'bg-red-500/20 text-red-400 border-red-500/30',
  paladin: 'bg-amber-500/20 text-amber-400 border-amber-500/30',
  sorcerer: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
  druid: 'bg-green-500/20 text-green-400 border-green-500/30',
  all: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
}

export default function SpellsPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedElement, setSelectedElement] = useState<SpellElement | 'all'>('all')
  const [selectedVocation, setSelectedVocation] = useState<Vocation | 'all'>('all')
  const [selectedType, setSelectedType] = useState<SpellType | 'all'>('all')
  const [sortBy, setSortBy] = useState<'name' | 'level' | 'mana' | 'cooldown'>('level')

  const filteredSpells = useMemo(() => {
    return spells
      .filter(spell => {
        const matchesSearch = spell.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                             spell.words.toLowerCase().includes(searchQuery.toLowerCase())
        const matchesElement = selectedElement === 'all' || spell.element === selectedElement
        const matchesVocation = selectedVocation === 'all' || spell.vocation.includes(selectedVocation) || spell.vocation.includes('all')
        const matchesType = selectedType === 'all' || spell.type === selectedType
        return matchesSearch && matchesElement && matchesVocation && matchesType
      })
      .sort((a, b) => {
        if (sortBy === 'name') return a.name.localeCompare(b.name)
        if (sortBy === 'level') return a.level - b.level
        if (sortBy === 'mana') return a.mana - b.mana
        if (sortBy === 'cooldown') return a.cooldown - b.cooldown
        return 0
      })
  }, [searchQuery, selectedElement, selectedVocation, selectedType, sortBy])

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
          {(['all', ...Object.keys(elementIcons)] as (SpellElement | 'all')[]).map((element) => {
            const Icon = element === 'all' ? Sparkles : elementIcons[element as SpellElement]
            const isActive = selectedElement === element
            return (
              <button
                key={element}
                onClick={() => setSelectedElement(element)}
                className={`flex items-center gap-2 px-4 py-2 rounded-xl border transition ${
                  isActive
                    ? element === 'all'
                      ? 'bg-white/10 border-white/30 text-white'
                      : `bg-gradient-to-r ${elementColors[element as SpellElement]} text-white border-transparent`
                    : 'bg-slate-800/50 border-slate-700/50 text-slate-400 hover:border-slate-600'
                }`}
              >
                <Icon className="w-4 h-4" />
                <span className="capitalize text-sm font-medium">
                  {element === 'all' ? 'All Elements' : element}
                </span>
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
            <option value="knight">Knight</option>
            <option value="paladin">Paladin</option>
            <option value="sorcerer">Sorcerer</option>
            <option value="druid">Druid</option>
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
            onChange={(e) => setSortBy(e.target.value as any)}
            className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-purple-500/50"
          >
            <option value="level">Sort by Level</option>
            <option value="name">Sort by Name</option>
            <option value="mana">Sort by Mana</option>
            <option value="cooldown">Sort by Cooldown</option>
          </select>

          <div className="text-slate-500 text-sm">
            {filteredSpells.length} spells found
          </div>
        </div>
      </section>

      {/* Spells Grid */}
      <section className="container mx-auto px-4 pb-20">
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
          {filteredSpells.map((spell, idx) => {
            const ElementIcon = elementIcons[spell.element]
            return (
              <motion.div
                key={spell.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: idx * 0.02 }}
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
                        {spell.damage} dmg
                      </span>
                    )}
                    {spell.healing && (
                      <span className="px-2 py-1 bg-green-500/20 text-green-400 text-xs rounded flex items-center gap-1">
                        <Heart className="w-3 h-3" />
                        {spell.healing} heal
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
                      <span key={voc} className={`px-2 py-0.5 text-xs rounded border ${vocationColors[voc]}`}>
                        {voc === 'all' ? 'All Vocations' : voc.charAt(0).toUpperCase() + voc.slice(1)}
                      </span>
                    ))}
                  </div>
                </div>
              </motion.div>
            )
          })}
        </div>

        {filteredSpells.length === 0 && (
          <div className="text-center py-20">
            <Sparkles className="w-12 h-12 text-slate-600 mx-auto mb-4" />
            <p className="text-slate-400">No spells found matching your criteria</p>
          </div>
        )}
      </section>
    </main>
  )
}

