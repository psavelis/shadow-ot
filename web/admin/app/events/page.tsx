'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { Calendar, Plus, Clock, Users, Trophy, Sparkles, Play, Pause, Edit, Trash2 } from 'lucide-react'

const events = [
  { id: '1', name: 'Winter Event 2024', type: 'seasonal', status: 'active', startAt: '2024-12-01T00:00:00Z', endAt: '2025-01-15T23:59:59Z', realm: null, participants: 4523 },
  { id: '2', name: 'Double XP Weekend', type: 'double_exp', status: 'scheduled', startAt: '2024-12-07T00:00:00Z', endAt: '2024-12-08T23:59:59Z', realm: null, participants: 0 },
  { id: '3', name: 'Boss Rush: Frost King', type: 'world_boss', status: 'scheduled', startAt: '2024-12-06T20:00:00Z', endAt: '2024-12-06T22:00:00Z', realm: 'aetheria', participants: 0 },
  { id: '4', name: 'PvP Tournament Season 5', type: 'pvp_tournament', status: 'active', startAt: '2024-11-15T00:00:00Z', endAt: '2024-12-31T23:59:59Z', realm: 'warbound', participants: 892 },
  { id: '5', name: 'Rapid Respawn', type: 'rapid_respawn', status: 'ended', startAt: '2024-11-25T00:00:00Z', endAt: '2024-11-26T23:59:59Z', realm: null, participants: 3456 },
]

const eventTypeConfig: Record<string, { color: string; icon: typeof Calendar }> = {
  seasonal: { color: 'bg-purple-500/20 text-purple-400', icon: Sparkles },
  double_exp: { color: 'bg-green-500/20 text-green-400', icon: Trophy },
  world_boss: { color: 'bg-red-500/20 text-red-400', icon: Users },
  pvp_tournament: { color: 'bg-orange-500/20 text-orange-400', icon: Trophy },
  rapid_respawn: { color: 'bg-blue-500/20 text-blue-400', icon: Clock },
  community: { color: 'bg-yellow-500/20 text-yellow-400', icon: Users },
}

const statusConfig = {
  active: { color: 'bg-green-500/20 text-green-400', label: 'Active' },
  scheduled: { color: 'bg-blue-500/20 text-blue-400', label: 'Scheduled' },
  ended: { color: 'bg-shadow-600/50 text-shadow-400', label: 'Ended' },
}

function formatDate(date: string) {
  return new Date(date).toLocaleString('en-US', { 
    month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' 
  })
}

export default function EventsPage() {
  const [filter, setFilter] = useState<'all' | 'active' | 'scheduled' | 'ended'>('all')

  const filteredEvents = events.filter(e => filter === 'all' || e.status === filter)

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-display font-bold mb-1">Event Management</h1>
            <p className="text-shadow-400">Schedule and manage game events</p>
          </div>
          <button className="btn-primary flex items-center gap-2">
            <Plus className="w-4 h-4" />
            Create Event
          </button>
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }} className="grid sm:grid-cols-4 gap-4">
        <div className="stat-card">
          <Calendar className="w-5 h-5 text-accent-500 mb-2" />
          <h3 className="text-2xl font-bold text-white">{events.length}</h3>
          <p className="text-shadow-400 text-sm">Total Events</p>
        </div>
        <div className="stat-card">
          <Play className="w-5 h-5 text-green-500 mb-2" />
          <h3 className="text-2xl font-bold text-white">{events.filter(e => e.status === 'active').length}</h3>
          <p className="text-shadow-400 text-sm">Active Now</p>
        </div>
        <div className="stat-card">
          <Clock className="w-5 h-5 text-blue-500 mb-2" />
          <h3 className="text-2xl font-bold text-white">{events.filter(e => e.status === 'scheduled').length}</h3>
          <p className="text-shadow-400 text-sm">Upcoming</p>
        </div>
        <div className="stat-card">
          <Users className="w-5 h-5 text-purple-500 mb-2" />
          <h3 className="text-2xl font-bold text-white">{events.reduce((a, b) => a + b.participants, 0).toLocaleString()}</h3>
          <p className="text-shadow-400 text-sm">Total Participants</p>
        </div>
      </motion.div>

      {/* Filters */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.15 }}>
        <div className="flex gap-2">
          {(['all', 'active', 'scheduled', 'ended'] as const).map((status) => (
            <button
              key={status}
              onClick={() => setFilter(status)}
              className={`px-4 py-2 rounded-lg text-sm font-medium capitalize transition-colors ${
                filter === status
                  ? 'bg-accent-500 text-white'
                  : 'bg-shadow-800 text-shadow-400 hover:text-white'
              }`}
            >
              {status}
            </button>
          ))}
        </div>
      </motion.div>

      {/* Events List */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }} className="space-y-4">
        {filteredEvents.map((event) => {
          const typeConfig = eventTypeConfig[event.type]
          const status = statusConfig[event.status]
          const Icon = typeConfig?.icon || Calendar

          return (
            <div key={event.id} className="card">
              <div className="flex items-center gap-6">
                <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${typeConfig?.color || 'bg-shadow-700'}`}>
                  <Icon className="w-6 h-6" />
                </div>
                <div className="flex-1">
                  <div className="flex items-center gap-3 mb-1">
                    <h3 className="text-lg font-semibold text-white">{event.name}</h3>
                    <span className={`px-2 py-0.5 rounded text-xs font-medium ${status.color}`}>
                      {status.label}
                    </span>
                    <span className={`px-2 py-0.5 rounded text-xs font-medium capitalize ${typeConfig?.color || 'bg-shadow-700 text-shadow-400'}`}>
                      {event.type.replace('_', ' ')}
                    </span>
                    {event.realm && (
                      <span className="px-2 py-0.5 rounded text-xs font-medium bg-shadow-700 text-shadow-400 capitalize">
                        {event.realm}
                      </span>
                    )}
                  </div>
                  <div className="flex items-center gap-4 text-sm text-shadow-400">
                    <span className="flex items-center gap-1">
                      <Clock className="w-4 h-4" />
                      {formatDate(event.startAt)} - {formatDate(event.endAt)}
                    </span>
                    {event.participants > 0 && (
                      <span className="flex items-center gap-1">
                        <Users className="w-4 h-4" />
                        {event.participants.toLocaleString()} participants
                      </span>
                    )}
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  {event.status === 'active' && (
                    <button className="p-2 text-yellow-400 hover:bg-yellow-500/20 rounded-lg transition-colors" title="Pause">
                      <Pause className="w-4 h-4" />
                    </button>
                  )}
                  {event.status === 'scheduled' && (
                    <button className="p-2 text-green-400 hover:bg-green-500/20 rounded-lg transition-colors" title="Start Now">
                      <Play className="w-4 h-4" />
                    </button>
                  )}
                  <button className="p-2 text-blue-400 hover:bg-blue-500/20 rounded-lg transition-colors" title="Edit">
                    <Edit className="w-4 h-4" />
                  </button>
                  <button className="p-2 text-red-400 hover:bg-red-500/20 rounded-lg transition-colors" title="Delete">
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              </div>
            </div>
          )
        })}
      </motion.div>
    </div>
  )
}

