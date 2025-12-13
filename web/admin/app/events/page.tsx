'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Calendar, Plus, Clock, Users, Trophy, Sparkles, Play, Pause, Edit, Trash2,
  Loader2, AlertTriangle
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'
import { useAdminEvents, useCreateEvent, useUpdateEvent, useDeleteEvent } from '@shadow-ot/shared'
import type { GameEvent, RealmId } from '@shadow-ot/shared'

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
  ended: { color: 'bg-slate-600/50 text-slate-400', label: 'Ended' },
}

function formatDate(date: string) {
  return new Date(date).toLocaleString('en-US', { 
    month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' 
  })
}

export default function EventsPage() {
  const [filter, setFilter] = useState<'all' | 'active' | 'scheduled' | 'ended'>('all')
  const [showCreateModal, setShowCreateModal] = useState(false)
  const [eventForm, setEventForm] = useState({
    name: '',
    type: 'seasonal' as GameEvent['type'],
    startAt: '',
    endAt: '',
    description: '',
    realm: '' as RealmId | '',
  })
  
  // Real hooks
  const { data: events = [], isLoading, error, refetch } = useAdminEvents(
    filter === 'all' ? {} : { status: filter as 'active' | 'scheduled' | 'ended' }
  )
  const createEvent = useCreateEvent()
  const updateEvent = useUpdateEvent()
  const deleteEvent = useDeleteEvent()

  const filteredEvents = events.filter((e: GameEvent) => filter === 'all' || e.status === filter)

  const handleCreateEvent = async () => {
    await createEvent.mutateAsync({
      name: eventForm.name,
      type: eventForm.type,
      startAt: eventForm.startAt,
      endAt: eventForm.endAt,
      description: eventForm.description,
      realm: (eventForm.realm || undefined) as RealmId | undefined,
      rewards: [],
    })
    setShowCreateModal(false)
    setEventForm({ name: '', type: 'seasonal', startAt: '', endAt: '', description: '', realm: '' })
  }

  const handleToggleStatus = async (event: GameEvent) => {
    const newStatus = event.status === 'active' ? 'scheduled' : 'active'
    await updateEvent.mutateAsync({ id: event.id, data: { status: newStatus } })
  }

  const handleDelete = async (id: string) => {
    if (confirm('Are you sure you want to delete this event?')) {
      await deleteEvent.mutateAsync(id)
    }
  }

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-white mb-1">Event Management</h1>
            <p className="text-slate-400">Schedule and manage game events</p>
          </div>
          <button 
            onClick={() => setShowCreateModal(true)}
            className="flex items-center gap-2 px-4 py-2 bg-purple-500 text-white rounded-lg hover:bg-purple-600 transition"
          >
            <Plus className="w-4 h-4" />
            Create Event
          </button>
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.1 }} 
        className="grid sm:grid-cols-4 gap-4"
      >
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Calendar className="w-5 h-5 text-purple-400 mb-2" />
          <h3 className="text-2xl font-bold text-white">{events.length}</h3>
          <p className="text-slate-400 text-sm">Total Events</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Play className="w-5 h-5 text-green-400 mb-2" />
          <h3 className="text-2xl font-bold text-white">{events.filter((e: GameEvent) => e.status === 'active').length}</h3>
          <p className="text-slate-400 text-sm">Active</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Clock className="w-5 h-5 text-blue-400 mb-2" />
          <h3 className="text-2xl font-bold text-white">{events.filter((e: GameEvent) => e.status === 'scheduled').length}</h3>
          <p className="text-slate-400 text-sm">Scheduled</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Users className="w-5 h-5 text-amber-400 mb-2" />
          <h3 className="text-2xl font-bold text-white">
            {events.reduce((sum: number, e: GameEvent) => sum + (e.participants || 0), 0).toLocaleString()}
          </h3>
          <p className="text-slate-400 text-sm">Total Participants</p>
        </div>
      </motion.div>

      {/* Filter Tabs */}
      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.2 }}
        className="flex gap-2"
      >
        {(['all', 'active', 'scheduled', 'ended'] as const).map((status) => (
          <button
            key={status}
            onClick={() => setFilter(status)}
            className={`px-4 py-2 rounded-lg transition capitalize ${
              filter === status
                ? 'bg-purple-500/20 text-purple-400'
                : 'bg-slate-800/50 text-slate-400 hover:bg-slate-700/50'
            }`}
          >
            {status}
          </button>
        ))}
      </motion.div>

      {/* Events List */}
      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.3 }}
        className="space-y-4"
      >
        {isLoading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-purple-400" />
          </div>
        ) : error ? (
          <div className="flex items-center justify-center py-12 text-red-400">
            <AlertTriangle className="w-5 h-5 mr-2" />
            Failed to load events
          </div>
        ) : filteredEvents.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12 text-slate-400 bg-slate-800/50 rounded-xl">
            <Calendar className="w-12 h-12 mb-4 opacity-50" />
            <p>No events found</p>
          </div>
        ) : (
          filteredEvents.map((event: GameEvent) => {
            const typeConf = eventTypeConfig[event.type] || eventTypeConfig.community
            const statusConf = statusConfig[event.status]
            const TypeIcon = typeConf.icon
            
            return (
              <div
                key={event.id}
                className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 hover:border-slate-600/50 transition"
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-4">
                    <div className={`p-3 rounded-lg ${typeConf.color.split(' ')[0]}`}>
                      <TypeIcon className={`w-5 h-5 ${typeConf.color.split(' ')[1]}`} />
                    </div>
                    <div>
                      <h3 className="text-white font-semibold flex items-center gap-2">
                        {event.name}
                        <span className={`px-2 py-0.5 text-xs rounded ${statusConf.color}`}>
                          {statusConf.label}
                        </span>
                      </h3>
                      <div className="flex items-center gap-4 mt-1 text-sm text-slate-400">
                        <span className="capitalize">{event.type.replace('_', ' ')}</span>
                        <span>|</span>
                        <span>{formatDate(event.startAt)} - {formatDate(event.endAt)}</span>
                        {event.realm && (
                          <>
                            <span>|</span>
                            <span className="text-purple-400">{event.realm}</span>
                          </>
                        )}
                      </div>
                    </div>
                  </div>
                  
                  <div className="flex items-center gap-4">
                    <div className="text-right">
                      <p className="text-white font-semibold">{(event.participants || 0).toLocaleString()}</p>
                      <p className="text-slate-500 text-sm">participants</p>
                    </div>
                    
                    <div className="flex items-center gap-2">
                      {event.status !== 'ended' && (
                        <button
                          onClick={() => handleToggleStatus(event)}
                          disabled={updateEvent.isPending}
                          className={`p-2 rounded-lg transition ${
                            event.status === 'active'
                              ? 'bg-amber-500/20 text-amber-400 hover:bg-amber-500/30'
                              : 'bg-green-500/20 text-green-400 hover:bg-green-500/30'
                          }`}
                          title={event.status === 'active' ? 'Pause' : 'Start'}
                        >
                          {event.status === 'active' ? (
                            <Pause className="w-4 h-4" />
                          ) : (
                            <Play className="w-4 h-4" />
                          )}
                        </button>
                      )}
                      <button
                        className="p-2 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-600/50 transition"
                        title="Edit"
                      >
                        <Edit className="w-4 h-4" />
                      </button>
                      <button
                        onClick={() => handleDelete(event.id)}
                        disabled={deleteEvent.isPending}
                        className="p-2 bg-red-500/20 text-red-400 rounded-lg hover:bg-red-500/30 transition"
                        title="Delete"
                      >
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            )
          })
        )}
      </motion.div>

      {/* Create Event Modal */}
      <Dialog.Root open={showCreateModal} onOpenChange={setShowCreateModal}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-lg bg-slate-900 border border-slate-700 rounded-xl p-6 z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-4 flex items-center gap-2">
              <Calendar className="w-6 h-6 text-purple-400" />
              Create Event
            </Dialog.Title>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm text-slate-400 mb-2">Event Name</label>
                <input
                  type="text"
                  value={eventForm.name}
                  onChange={(e) => setEventForm(f => ({ ...f, name: e.target.value }))}
                  placeholder="e.g. Winter Festival 2024"
                  className="w-full bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-purple-500/50"
                />
              </div>
              
              <div>
                <label className="block text-sm text-slate-400 mb-2">Event Type</label>
                <select
                  value={eventForm.type}
                  onChange={(e) => setEventForm(f => ({ ...f, type: e.target.value as GameEvent['type'] }))}
                  className="w-full bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-purple-500/50"
                >
                  <option value="seasonal">Seasonal</option>
                  <option value="double_exp">Double XP</option>
                  <option value="world_boss">World Boss</option>
                  <option value="pvp_tournament">PvP Tournament</option>
                  <option value="rapid_respawn">Rapid Respawn</option>
                  <option value="community">Community</option>
                </select>
              </div>
              
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm text-slate-400 mb-2">Start Date</label>
                  <input
                    type="datetime-local"
                    value={eventForm.startAt}
                    onChange={(e) => setEventForm(f => ({ ...f, startAt: e.target.value }))}
                    className="w-full bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-purple-500/50"
                  />
                </div>
                <div>
                  <label className="block text-sm text-slate-400 mb-2">End Date</label>
                  <input
                    type="datetime-local"
                    value={eventForm.endAt}
                    onChange={(e) => setEventForm(f => ({ ...f, endAt: e.target.value }))}
                    className="w-full bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-purple-500/50"
                  />
                </div>
              </div>
              
              <div>
                <label className="block text-sm text-slate-400 mb-2">Description</label>
                <textarea
                  value={eventForm.description}
                  onChange={(e) => setEventForm(f => ({ ...f, description: e.target.value }))}
                  placeholder="Event description..."
                  rows={3}
                  className="w-full bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-purple-500/50 resize-none"
                />
              </div>
            </div>
            
            <div className="flex gap-3 mt-6">
              <Dialog.Close asChild>
                <button className="flex-1 px-4 py-2 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-600/50 transition">
                  Cancel
                </button>
              </Dialog.Close>
              <button
                onClick={handleCreateEvent}
                disabled={!eventForm.name || !eventForm.startAt || !eventForm.endAt || createEvent.isPending}
                className="flex-1 px-4 py-2 bg-purple-500 text-white rounded-lg hover:bg-purple-600 transition disabled:opacity-50 flex items-center justify-center gap-2"
              >
                {createEvent.isPending ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    Creating...
                  </>
                ) : (
                  'Create Event'
                )}
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}
