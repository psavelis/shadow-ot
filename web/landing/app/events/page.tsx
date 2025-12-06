'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Calendar, Clock, Star, Flame, Trophy, Gift, Sparkles,
  ChevronLeft, ChevronRight, Users, MapPin, Swords, Crown,
  Zap, Target, PartyPopper, Timer, CalendarDays, Bell
} from 'lucide-react'

// Today's boosted creature and boss
const boostedCreature = {
  name: 'Dragon Lord',
  image: '/creatures/dragon-lord.png',
  loot: '+100%',
  experience: '+50%',
  spawn: 'Draconia, Ferumbras Tower',
  difficulty: 'Hard',
}

const boostedBoss = {
  name: 'Ghazbaran',
  image: '/bosses/ghazbaran.png',
  loot: '+200%',
  charm: '+50 points',
  spawn: 'The Pits of Inferno',
  difficulty: 'Extreme',
}

// Current and upcoming events
const events = [
  {
    id: '1',
    name: 'Double Experience Weekend',
    type: 'experience',
    status: 'active',
    startDate: '2024-12-06',
    endDate: '2024-12-08',
    description: 'Earn double experience from all monster kills this weekend!',
    multiplier: '2x',
    icon: Zap,
    color: 'from-purple-500 to-violet-600',
    realms: ['All Realms'],
  },
  {
    id: '2',
    name: 'Winter Solstice Festival',
    type: 'seasonal',
    status: 'active',
    startDate: '2024-12-01',
    endDate: '2024-12-31',
    description: 'Collect snowflakes, defeat the Ice King, and earn exclusive winter rewards!',
    icon: Sparkles,
    color: 'from-cyan-400 to-blue-500',
    realms: ['All Realms'],
    rewards: ['Ice Crown', 'Frost Wings', 'Winter Mount'],
  },
  {
    id: '3',
    name: 'PvP Arena Tournament',
    type: 'tournament',
    status: 'upcoming',
    startDate: '2024-12-14',
    endDate: '2024-12-15',
    description: 'Compete in the monthly PvP tournament for glory and exclusive rewards!',
    icon: Swords,
    color: 'from-red-500 to-orange-500',
    realms: ['Nova', 'Eldoria'],
    prizePool: '10,000,000 gold',
  },
  {
    id: '4',
    name: 'World Boss: Ancient Dragon',
    type: 'world_boss',
    status: 'upcoming',
    startDate: '2024-12-10',
    endDate: '2024-12-10',
    description: 'A legendary dragon awakens! Unite with other players to defeat it.',
    icon: Crown,
    color: 'from-amber-500 to-red-500',
    realms: ['Shadowlands'],
    rewards: ['Dragon Slayer Title', 'Ancient Scale Armor'],
  },
  {
    id: '5',
    name: 'Treasure Hunt',
    type: 'hunt',
    status: 'upcoming',
    startDate: '2024-12-20',
    endDate: '2024-12-22',
    description: 'Hidden chests spawn across all realms. Find them for rare loot!',
    icon: Gift,
    color: 'from-yellow-400 to-amber-500',
    realms: ['All Realms'],
  },
]

// World quests
const worldQuests = [
  {
    id: '1',
    name: 'The Demon\'s Forge',
    progress: 78,
    goal: 10000,
    current: 7823,
    description: 'Defeat demons in the Demon Forge to unlock the Infernal Armory.',
    reward: 'Access to Infernal Armory + Demon Slayer achievement',
    endsIn: '2 days',
    realm: 'Nova',
  },
  {
    id: '2',
    name: 'Ocean\'s Bounty',
    progress: 45,
    goal: 5000,
    current: 2250,
    description: 'Collect sea shells from Quara to cleanse the Coral Reef.',
    reward: '500k gold realm treasury + Coral Mount',
    endsIn: '5 days',
    realm: 'Eldoria',
  },
  {
    id: '3',
    name: 'The Undead Purge',
    progress: 92,
    goal: 20000,
    current: 18400,
    description: 'Eliminate undead creatures to seal the Lich King\'s tomb.',
    reward: 'Anti-Undead enchantment available for 7 days',
    endsIn: '6 hours',
    realm: 'Shadowlands',
  },
]

// Calendar days for December 2024
const calendarDays = Array.from({ length: 31 }, (_, i) => i + 1)
const eventDays = [1, 6, 7, 8, 10, 14, 15, 20, 21, 22, 25, 31] // Days with events

export default function EventsPage() {
  const [selectedMonth, setSelectedMonth] = useState(11) // December
  const [selectedYear, setSelectedYear] = useState(2024)

  const getEventTypeColor = (type: string) => {
    switch (type) {
      case 'experience': return 'bg-purple-500/20 text-purple-400 border-purple-500/30'
      case 'seasonal': return 'bg-cyan-500/20 text-cyan-400 border-cyan-500/30'
      case 'tournament': return 'bg-red-500/20 text-red-400 border-red-500/30'
      case 'world_boss': return 'bg-amber-500/20 text-amber-400 border-amber-500/30'
      case 'hunt': return 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30'
      default: return 'bg-slate-500/20 text-slate-400 border-slate-500/30'
    }
  }

  return (
    <main className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950">
      {/* Hero */}
      <section className="relative py-20 overflow-hidden">
        <div className="absolute inset-0 bg-[url('/grid.svg')] opacity-5" />
        <div className="absolute inset-0 bg-gradient-to-b from-amber-500/5 via-transparent to-transparent" />
        
        <div className="container mx-auto px-4 relative z-10">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="text-center max-w-3xl mx-auto"
          >
            <div className="inline-flex items-center gap-2 px-4 py-2 bg-amber-500/10 border border-amber-500/30 rounded-full text-amber-400 text-sm mb-6">
              <Calendar className="w-4 h-4" />
              Events & Calendar
            </div>
            <h1 className="text-4xl md:text-5xl font-bold text-white mb-4">
              Game <span className="text-transparent bg-clip-text bg-gradient-to-r from-amber-400 to-orange-500">Events</span>
            </h1>
            <p className="text-slate-400 text-lg">
              Boosted creatures, seasonal events, world quests, and tournaments. Never miss out!
            </p>
          </motion.div>
        </div>
      </section>

      {/* Today's Boosted */}
      <section className="container mx-auto px-4 -mt-8 mb-12">
        <div className="grid md:grid-cols-2 gap-6">
          {/* Boosted Creature */}
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            className="bg-gradient-to-br from-emerald-500/10 to-green-500/5 border border-emerald-500/30 rounded-2xl p-6"
          >
            <div className="flex items-center gap-2 mb-4">
              <div className="px-3 py-1 bg-emerald-500/20 rounded-full">
                <span className="text-emerald-400 text-sm font-medium flex items-center gap-1">
                  <Flame className="w-4 h-4" />
                  Today's Boosted Creature
                </span>
              </div>
              <div className="flex items-center gap-1 text-slate-500 text-sm">
                <Timer className="w-4 h-4" />
                Resets in 14h 32m
              </div>
            </div>
            
            <div className="flex items-center gap-6">
              <div className="w-24 h-24 bg-slate-800/50 rounded-xl flex items-center justify-center border border-slate-700/50">
                <Target className="w-12 h-12 text-emerald-400" />
              </div>
              <div className="flex-1">
                <h3 className="text-2xl font-bold text-white mb-1">{boostedCreature.name}</h3>
                <p className="text-slate-400 text-sm mb-3">{boostedCreature.spawn}</p>
                <div className="flex flex-wrap gap-2">
                  <span className="px-3 py-1 bg-amber-500/20 text-amber-400 rounded-lg text-sm font-medium">
                    Loot {boostedCreature.loot}
                  </span>
                  <span className="px-3 py-1 bg-purple-500/20 text-purple-400 rounded-lg text-sm font-medium">
                    XP {boostedCreature.experience}
                  </span>
                  <span className="px-3 py-1 bg-red-500/20 text-red-400 rounded-lg text-sm font-medium">
                    {boostedCreature.difficulty}
                  </span>
                </div>
              </div>
            </div>
          </motion.div>

          {/* Boosted Boss */}
          <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            className="bg-gradient-to-br from-purple-500/10 to-pink-500/5 border border-purple-500/30 rounded-2xl p-6"
          >
            <div className="flex items-center gap-2 mb-4">
              <div className="px-3 py-1 bg-purple-500/20 rounded-full">
                <span className="text-purple-400 text-sm font-medium flex items-center gap-1">
                  <Crown className="w-4 h-4" />
                  Today's Boosted Boss
                </span>
              </div>
              <div className="flex items-center gap-1 text-slate-500 text-sm">
                <Timer className="w-4 h-4" />
                Resets in 14h 32m
              </div>
            </div>
            
            <div className="flex items-center gap-6">
              <div className="w-24 h-24 bg-slate-800/50 rounded-xl flex items-center justify-center border border-slate-700/50">
                <Crown className="w-12 h-12 text-purple-400" />
              </div>
              <div className="flex-1">
                <h3 className="text-2xl font-bold text-white mb-1">{boostedBoss.name}</h3>
                <p className="text-slate-400 text-sm mb-3">{boostedBoss.spawn}</p>
                <div className="flex flex-wrap gap-2">
                  <span className="px-3 py-1 bg-amber-500/20 text-amber-400 rounded-lg text-sm font-medium">
                    Loot {boostedBoss.loot}
                  </span>
                  <span className="px-3 py-1 bg-cyan-500/20 text-cyan-400 rounded-lg text-sm font-medium">
                    Charm {boostedBoss.charm}
                  </span>
                  <span className="px-3 py-1 bg-red-500/20 text-red-400 rounded-lg text-sm font-medium">
                    {boostedBoss.difficulty}
                  </span>
                </div>
              </div>
            </div>
          </motion.div>
        </div>
      </section>

      {/* Active & Upcoming Events */}
      <section className="container mx-auto px-4 mb-12">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-2xl font-bold text-white flex items-center gap-2">
            <PartyPopper className="w-6 h-6 text-amber-400" />
            Events
          </h2>
          <button className="flex items-center gap-2 text-amber-400 hover:text-amber-300 transition text-sm">
            <Bell className="w-4 h-4" />
            Subscribe to notifications
          </button>
        </div>

        <div className="grid lg:grid-cols-2 gap-4">
          {events.map((event, idx) => {
            const Icon = event.icon
            return (
              <motion.div
                key={event.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: idx * 0.05 }}
                className={`relative overflow-hidden rounded-xl border ${
                  event.status === 'active' 
                    ? 'border-emerald-500/50 bg-gradient-to-br from-emerald-500/5 to-transparent' 
                    : 'border-slate-700/50 bg-slate-800/50'
                }`}
              >
                {event.status === 'active' && (
                  <div className="absolute top-4 right-4">
                    <span className="flex items-center gap-1 px-2 py-1 bg-emerald-500/20 text-emerald-400 text-xs rounded-full">
                      <span className="w-2 h-2 bg-emerald-400 rounded-full animate-pulse" />
                      Active Now
                    </span>
                  </div>
                )}
                
                <div className="p-6">
                  <div className="flex items-start gap-4">
                    <div className={`w-14 h-14 rounded-xl bg-gradient-to-br ${event.color} flex items-center justify-center shrink-0`}>
                      <Icon className="w-7 h-7 text-white" />
                    </div>
                    <div className="flex-1">
                      <h3 className="text-xl font-bold text-white mb-1">{event.name}</h3>
                      <p className="text-slate-400 text-sm mb-3">{event.description}</p>
                      
                      <div className="flex flex-wrap gap-2 mb-3">
                        <span className={`px-2 py-1 text-xs rounded border ${getEventTypeColor(event.type)}`}>
                          {event.type.replace('_', ' ').toUpperCase()}
                        </span>
                        {event.multiplier && (
                          <span className="px-2 py-1 bg-purple-500/20 text-purple-400 text-xs rounded border border-purple-500/30">
                            {event.multiplier} XP
                          </span>
                        )}
                        {event.prizePool && (
                          <span className="px-2 py-1 bg-amber-500/20 text-amber-400 text-xs rounded border border-amber-500/30">
                            Prize: {event.prizePool}
                          </span>
                        )}
                      </div>

                      {event.rewards && (
                        <div className="flex flex-wrap gap-1 mb-3">
                          {event.rewards.map((reward, i) => (
                            <span key={i} className="px-2 py-0.5 bg-slate-700/50 text-slate-300 text-xs rounded">
                              {reward}
                            </span>
                          ))}
                        </div>
                      )}

                      <div className="flex items-center gap-4 text-xs text-slate-500">
                        <span className="flex items-center gap-1">
                          <CalendarDays className="w-3 h-3" />
                          {event.startDate} - {event.endDate}
                        </span>
                        <span className="flex items-center gap-1">
                          <MapPin className="w-3 h-3" />
                          {event.realms.join(', ')}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              </motion.div>
            )
          })}
        </div>
      </section>

      {/* World Quests */}
      <section className="container mx-auto px-4 mb-12">
        <h2 className="text-2xl font-bold text-white flex items-center gap-2 mb-6">
          <Users className="w-6 h-6 text-cyan-400" />
          Active World Quests
        </h2>

        <div className="grid lg:grid-cols-3 gap-4">
          {worldQuests.map((quest, idx) => (
            <motion.div
              key={quest.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: idx * 0.05 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
            >
              <div className="flex items-center justify-between mb-4">
                <span className="px-2 py-1 bg-cyan-500/20 text-cyan-400 text-xs rounded border border-cyan-500/30">
                  {quest.realm}
                </span>
                <span className="text-amber-400 text-sm flex items-center gap-1">
                  <Clock className="w-3 h-3" />
                  {quest.endsIn}
                </span>
              </div>

              <h3 className="text-lg font-bold text-white mb-2">{quest.name}</h3>
              <p className="text-slate-400 text-sm mb-4">{quest.description}</p>

              {/* Progress Bar */}
              <div className="mb-3">
                <div className="flex items-center justify-between text-sm mb-1">
                  <span className="text-slate-400">Progress</span>
                  <span className="text-white font-medium">{quest.current.toLocaleString()} / {quest.goal.toLocaleString()}</span>
                </div>
                <div className="h-3 bg-slate-700/50 rounded-full overflow-hidden">
                  <div 
                    className={`h-full rounded-full transition-all ${
                      quest.progress >= 90 ? 'bg-gradient-to-r from-emerald-500 to-green-400' :
                      quest.progress >= 50 ? 'bg-gradient-to-r from-amber-500 to-yellow-400' :
                      'bg-gradient-to-r from-cyan-500 to-blue-400'
                    }`}
                    style={{ width: `${quest.progress}%` }}
                  />
                </div>
                <p className="text-right text-xs text-slate-500 mt-1">{quest.progress}% complete</p>
              </div>

              <div className="p-3 bg-slate-900/50 rounded-lg">
                <p className="text-xs text-slate-500 mb-1">Reward:</p>
                <p className="text-sm text-emerald-400">{quest.reward}</p>
              </div>
            </motion.div>
          ))}
        </div>
      </section>

      {/* Calendar */}
      <section className="container mx-auto px-4 pb-20">
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden">
          <div className="p-4 border-b border-slate-700 flex items-center justify-between">
            <h2 className="text-white font-bold flex items-center gap-2">
              <CalendarDays className="w-5 h-5 text-amber-400" />
              Event Calendar
            </h2>
            <div className="flex items-center gap-4">
              <button className="p-2 text-slate-400 hover:text-white transition">
                <ChevronLeft className="w-5 h-5" />
              </button>
              <span className="text-white font-medium">December 2024</span>
              <button className="p-2 text-slate-400 hover:text-white transition">
                <ChevronRight className="w-5 h-5" />
              </button>
            </div>
          </div>

          <div className="p-4">
            {/* Days header */}
            <div className="grid grid-cols-7 gap-2 mb-2">
              {['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'].map(day => (
                <div key={day} className="text-center text-slate-500 text-sm py-2">
                  {day}
                </div>
              ))}
            </div>

            {/* Calendar grid */}
            <div className="grid grid-cols-7 gap-2">
              {/* Empty cells for December 2024 starting on Sunday */}
              {calendarDays.map(day => {
                const hasEvent = eventDays.includes(day)
                const isToday = day === 6 // Assuming today is Dec 6
                return (
                  <div
                    key={day}
                    className={`aspect-square flex flex-col items-center justify-center rounded-lg text-sm transition cursor-pointer ${
                      isToday 
                        ? 'bg-amber-500 text-white font-bold' 
                        : hasEvent 
                          ? 'bg-purple-500/20 text-purple-400 hover:bg-purple-500/30 border border-purple-500/30' 
                          : 'bg-slate-900/50 text-slate-400 hover:bg-slate-700/50'
                    }`}
                  >
                    <span>{day}</span>
                    {hasEvent && !isToday && (
                      <span className="w-1.5 h-1.5 bg-purple-400 rounded-full mt-1" />
                    )}
                  </div>
                )
              })}
            </div>
          </div>

          <div className="p-4 border-t border-slate-700 flex items-center gap-4 text-xs">
            <span className="flex items-center gap-2">
              <span className="w-3 h-3 bg-amber-500 rounded" />
              Today
            </span>
            <span className="flex items-center gap-2">
              <span className="w-3 h-3 bg-purple-500/50 rounded border border-purple-500/50" />
              Has Events
            </span>
          </div>
        </div>
      </section>
    </main>
  )
}

