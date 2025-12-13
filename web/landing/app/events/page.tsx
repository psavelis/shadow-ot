'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Calendar, Clock, Star, Flame, Trophy, Gift, Sparkles,
  ChevronLeft, ChevronRight, Users, MapPin, Swords, Crown,
  Zap, Target, PartyPopper, Timer, CalendarDays, Bell,
  Loader2, AlertCircle
} from 'lucide-react'
import { useDailyBoosted, useActiveWorldQuests, eventApi, getCreatureSprite, getTimeUntilReset } from '@shadow-ot/shared'
import { useQuery } from '@tanstack/react-query'
import type { GameEvent, RealmId } from '@shadow-ot/shared'

// Event type icons mapping
const eventTypeIcons: Record<string, React.ComponentType<{ className?: string }>> = {
  double_exp: Zap,
  rapid_respawn: Timer,
  world_boss: Crown,
  invasion: Swords,
  seasonal: Sparkles,
  pvp_tournament: Trophy,
  community: Users,
}

const eventTypeColors: Record<string, string> = {
  double_exp: 'from-purple-500 to-violet-600',
  rapid_respawn: 'from-green-500 to-emerald-600',
  world_boss: 'from-amber-500 to-red-500',
  invasion: 'from-red-500 to-rose-600',
  seasonal: 'from-cyan-400 to-blue-500',
  pvp_tournament: 'from-red-500 to-orange-500',
  community: 'from-blue-500 to-indigo-600',
}

export default function EventsPage() {
  const [selectedMonth] = useState(new Date().getMonth())
  const [selectedYear] = useState(new Date().getFullYear())

  // Fetch real data from API
  const { creature: boostedCreature, boss: boostedBoss, isLoading: boostedLoading, error: boostedError } = useDailyBoosted()
  
  const { data: worldQuests, isLoading: questsLoading } = useActiveWorldQuests()
  
  const { data: activeEvents, isLoading: eventsLoading } = useQuery({
    queryKey: ['events', 'active'],
    queryFn: () => eventApi.getActive(),
    staleTime: 1000 * 60 * 5,
  })

  const { data: upcomingEvents, isLoading: upcomingLoading } = useQuery({
    queryKey: ['events', 'upcoming'],
    queryFn: () => eventApi.getUpcoming(),
    staleTime: 1000 * 60 * 5,
  })

  const allEvents = [...(activeEvents || []), ...(upcomingEvents || [])]

  const getEventTypeColor = (type: string) => {
    const colors: Record<string, string> = {
      experience: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
      seasonal: 'bg-cyan-500/20 text-cyan-400 border-cyan-500/30',
      tournament: 'bg-red-500/20 text-red-400 border-red-500/30',
      world_boss: 'bg-amber-500/20 text-amber-400 border-amber-500/30',
      hunt: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
    }
    return colors[type] || 'bg-slate-500/20 text-slate-400 border-slate-500/30'
  }

  // Generate calendar days
  const daysInMonth = new Date(selectedYear, selectedMonth + 1, 0).getDate()
  const firstDayOfMonth = new Date(selectedYear, selectedMonth, 1).getDay()
  const calendarDays = Array.from({ length: daysInMonth }, (_, i) => i + 1)
  const today = new Date().getDate()
  const isCurrentMonth = selectedMonth === new Date().getMonth() && selectedYear === new Date().getFullYear()

  // Get days with events
  const eventDays = allEvents.map(event => new Date(event.startAt).getDate())

  const timeUntilReset = getTimeUntilReset()

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
                Resets in {timeUntilReset}
              </div>
            </div>
            
            {boostedLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="w-8 h-8 animate-spin text-emerald-400" />
              </div>
            ) : boostedError ? (
              <div className="flex items-center justify-center gap-2 py-8 text-red-400">
                <AlertCircle className="w-5 h-5" />
                <span>Failed to load</span>
              </div>
            ) : boostedCreature && (
              <div className="flex items-center gap-6">
                <div className="w-24 h-24 bg-slate-800/50 rounded-xl flex items-center justify-center border border-slate-700/50 overflow-hidden">
                  <img 
                    src={getCreatureSprite(boostedCreature.name)} 
                    alt={boostedCreature.name}
                    className="w-16 h-16 object-contain"
                    onError={(e) => {
                      (e.target as HTMLImageElement).src = '/icons/creatures/default.png'
                    }}
                  />
                </div>
                <div className="flex-1">
                  <h3 className="text-2xl font-bold text-white mb-1">{boostedCreature.name}</h3>
                  <p className="text-slate-400 text-sm mb-3">{boostedCreature.spawn}</p>
                  <div className="flex flex-wrap gap-2">
                    <span className="px-3 py-1 bg-amber-500/20 text-amber-400 rounded-lg text-sm font-medium">
                      Loot +{boostedCreature.lootBonus}%
                    </span>
                    <span className="px-3 py-1 bg-purple-500/20 text-purple-400 rounded-lg text-sm font-medium">
                      XP +{boostedCreature.experienceBonus}%
                    </span>
                    <span className="px-3 py-1 bg-red-500/20 text-red-400 rounded-lg text-sm font-medium">
                      {boostedCreature.difficulty}
                    </span>
                  </div>
                </div>
              </div>
            )}
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
                Resets in {timeUntilReset}
              </div>
            </div>
            
            {boostedLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="w-8 h-8 animate-spin text-purple-400" />
              </div>
            ) : boostedError ? (
              <div className="flex items-center justify-center gap-2 py-8 text-red-400">
                <AlertCircle className="w-5 h-5" />
                <span>Failed to load</span>
              </div>
            ) : boostedBoss && (
              <div className="flex items-center gap-6">
                <div className="w-24 h-24 bg-slate-800/50 rounded-xl flex items-center justify-center border border-slate-700/50 overflow-hidden">
                  <img 
                    src={getCreatureSprite(boostedBoss.name)} 
                    alt={boostedBoss.name}
                    className="w-16 h-16 object-contain"
                    onError={(e) => {
                      (e.target as HTMLImageElement).src = '/icons/bosses/default.png'
                    }}
                  />
                </div>
                <div className="flex-1">
                  <h3 className="text-2xl font-bold text-white mb-1">{boostedBoss.name}</h3>
                  <p className="text-slate-400 text-sm mb-3">{boostedBoss.spawn}</p>
                  <div className="flex flex-wrap gap-2">
                    <span className="px-3 py-1 bg-amber-500/20 text-amber-400 rounded-lg text-sm font-medium">
                      Loot +{boostedBoss.lootBonus}%
                    </span>
                    <span className="px-3 py-1 bg-cyan-500/20 text-cyan-400 rounded-lg text-sm font-medium">
                      Charm +{boostedBoss.charmBonus} pts
                    </span>
                    <span className="px-3 py-1 bg-red-500/20 text-red-400 rounded-lg text-sm font-medium">
                      {boostedBoss.difficulty}
                    </span>
                  </div>
                </div>
              </div>
            )}
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

        {eventsLoading || upcomingLoading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-10 h-10 animate-spin text-amber-400" />
          </div>
        ) : allEvents.length > 0 ? (
          <div className="grid lg:grid-cols-2 gap-4">
            {allEvents.map((event, idx) => {
              const Icon = eventTypeIcons[event.type] || PartyPopper
              const colorClass = eventTypeColors[event.type] || 'from-slate-500 to-slate-600'
              const isActive = event.status === 'active'
              
              return (
                <motion.div
                  key={event.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: idx * 0.05 }}
                  className={`relative overflow-hidden rounded-xl border ${
                    isActive 
                      ? 'border-emerald-500/50 bg-gradient-to-br from-emerald-500/5 to-transparent' 
                      : 'border-slate-700/50 bg-slate-800/50'
                  }`}
                >
                  {isActive && (
                    <div className="absolute top-4 right-4">
                      <span className="flex items-center gap-1 px-2 py-1 bg-emerald-500/20 text-emerald-400 text-xs rounded-full">
                        <span className="w-2 h-2 bg-emerald-400 rounded-full animate-pulse" />
                        Active Now
                      </span>
                    </div>
                  )}
                  
                  <div className="p-6">
                    <div className="flex items-start gap-4">
                      <div className={`w-14 h-14 rounded-xl bg-gradient-to-br ${colorClass} flex items-center justify-center shrink-0`}>
                        <Icon className="w-7 h-7 text-white" />
                      </div>
                      <div className="flex-1">
                        <h3 className="text-xl font-bold text-white mb-1">{event.name}</h3>
                        <p className="text-slate-400 text-sm mb-3">{event.description}</p>
                        
                        <div className="flex flex-wrap gap-2 mb-3">
                          <span className={`px-2 py-1 text-xs rounded border ${getEventTypeColor(event.type)}`}>
                            {event.type.replace('_', ' ').toUpperCase()}
                          </span>
                        </div>

                        {event.rewards && event.rewards.length > 0 && (
                          <div className="flex flex-wrap gap-1 mb-3">
                            {event.rewards.map((reward, i) => (
                              <span key={i} className="px-2 py-0.5 bg-slate-700/50 text-slate-300 text-xs rounded">
                                {typeof reward.value === 'string' ? reward.value : `${reward.type}: ${reward.value}`}
                              </span>
                            ))}
                          </div>
                        )}

                        <div className="flex items-center gap-4 text-xs text-slate-500">
                          <span className="flex items-center gap-1">
                            <CalendarDays className="w-3 h-3" />
                            {new Date(event.startAt).toLocaleDateString()} - {new Date(event.endAt).toLocaleDateString()}
                          </span>
                          {event.realm && (
                            <span className="flex items-center gap-1">
                              <MapPin className="w-3 h-3" />
                              {event.realm}
                            </span>
                          )}
                        </div>
                      </div>
                    </div>
                  </div>
                </motion.div>
              )
            })}
          </div>
        ) : (
          <div className="text-center py-12 text-slate-400">
            <PartyPopper className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>No active events at the moment</p>
          </div>
        )}
      </section>

      {/* World Quests */}
      <section className="container mx-auto px-4 mb-12">
        <h2 className="text-2xl font-bold text-white flex items-center gap-2 mb-6">
          <Users className="w-6 h-6 text-cyan-400" />
          Active World Quests
        </h2>

        {questsLoading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-10 h-10 animate-spin text-cyan-400" />
          </div>
        ) : worldQuests && worldQuests.length > 0 ? (
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
                  <span className="px-2 py-1 bg-cyan-500/20 text-cyan-400 text-xs rounded border border-cyan-500/30 capitalize">
                    {quest.realm}
                  </span>
                  <span className="text-amber-400 text-sm flex items-center gap-1">
                    <Clock className="w-3 h-3" />
                    {new Date(quest.endsAt).toLocaleDateString()}
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
        ) : (
          <div className="text-center py-12 text-slate-400">
            <Users className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>No active world quests</p>
          </div>
        )}
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
              <span className="text-white font-medium">
                {new Date(selectedYear, selectedMonth).toLocaleString('default', { month: 'long', year: 'numeric' })}
              </span>
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
              {/* Empty cells for offset */}
              {Array.from({ length: firstDayOfMonth }, (_, i) => (
                <div key={`empty-${i}`} className="aspect-square" />
              ))}
              
              {calendarDays.map(day => {
                const hasEvent = eventDays.includes(day)
                const isToday = isCurrentMonth && day === today
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
