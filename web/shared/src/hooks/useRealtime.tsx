import { useEffect, useRef, useCallback, useState, createContext, useContext, ReactNode } from 'react'
import { io, Socket } from 'socket.io-client'
import { apiClient, WS_BASE_URL } from '../api/client'
import { RealtimeEvent, RealtimeEventType } from '../types'

// ====== Types ======

interface UseRealtimeOptions {
  autoConnect?: boolean
  reconnect?: boolean
  events?: RealtimeEventType[]
}

interface UseRealtimeReturn {
  isConnected: boolean
  latency: number
  connect: () => void
  disconnect: () => void
  subscribe: <T = unknown>(event: RealtimeEventType, callback: (payload: T) => void) => () => void
  emit: (event: string, data: unknown) => void
  joinRoom: (room: string) => void
  leaveRoom: (room: string) => void
}

interface RealtimeContextValue extends UseRealtimeReturn {
  serverStatus: ServerStatus | null
}

interface ServerStatus {
  playersOnline: number
  realmsOnline: number
  uptime: number
  version: string
}

interface PlayerLoginPayload {
  characterId: number
  characterName: string
  level: number
  vocation: string
  realm: string
}

interface PlayerLogoutPayload {
  characterId: number
  characterName: string
  realm: string
}

interface LevelUpPayload {
  characterId: number
  characterName: string
  oldLevel: number
  newLevel: number
  realm: string
}

interface DeathPayload {
  characterId: number
  characterName: string
  level: number
  killedBy: string
  isPlayer: boolean
  realm: string
}

interface AchievementPayload {
  characterId: number
  characterName: string
  achievementId: number
  achievementName: string
  points: number
}

interface ChatMessagePayload {
  room: string
  userId: number
  username: string
  message: string
  timestamp: number
}

interface RealmStatusPayload {
  realmId: string
  realmName: string
  status: 'online' | 'offline' | 'maintenance'
  playersOnline: number
}

interface MarketUpdatePayload {
  itemId: number
  itemName: string
  price: number
  change: number
  volume: number
}

interface GuildWarPayload {
  guildA: { id: number; name: string; kills: number }
  guildB: { id: number; name: string; kills: number }
  realm: string
  status: 'started' | 'ended' | 'update'
}

// ====== Context ======

const RealtimeContext = createContext<RealtimeContextValue | null>(null)

export function RealtimeProvider({ children }: { children: ReactNode }) {
  const realtime = useRealtime()
  const [serverStatus, setServerStatus] = useState<ServerStatus | null>(null)

  useEffect(() => {
    const unsubscribe = realtime.subscribe<ServerStatus>('server.status', (status) => {
      setServerStatus(status)
    })
    return unsubscribe
  }, [realtime])

  return (
    <RealtimeContext.Provider value={{ ...realtime, serverStatus }}>
      {children}
    </RealtimeContext.Provider>
  )
}

export function useRealtimeContext() {
  const context = useContext(RealtimeContext)
  if (!context) {
    throw new Error('useRealtimeContext must be used within RealtimeProvider')
  }
  return context
}

// ====== Main Hook ======

export function useRealtime(options: UseRealtimeOptions = {}): UseRealtimeReturn {
  const { autoConnect = true, reconnect = true } = options
  const socketRef = useRef<Socket | null>(null)
  const [isConnected, setIsConnected] = useState(false)
  const [latency, setLatency] = useState(0)
  const listenersRef = useRef<Map<string, Set<(payload: unknown) => void>>>(new Map())
  const pingIntervalRef = useRef<NodeJS.Timeout | null>(null)

  const connect = useCallback(() => {
    if (socketRef.current?.connected) return

    const token = apiClient.getAccessToken()
    
    socketRef.current = io(WS_BASE_URL, {
      auth: token ? { token } : undefined,
      transports: ['websocket'],
      reconnection: reconnect,
      reconnectionAttempts: 5,
      reconnectionDelay: 1000,
    })

    socketRef.current.on('connect', () => {
      setIsConnected(true)
      console.log('[WS] Connected')
      
      // Start ping interval
      pingIntervalRef.current = setInterval(() => {
        const start = Date.now()
        socketRef.current?.emit('ping', {}, () => {
          setLatency(Date.now() - start)
        })
      }, 5000)
    })

    socketRef.current.on('disconnect', (reason) => {
      setIsConnected(false)
      console.log('[WS] Disconnected:', reason)
      
      if (pingIntervalRef.current) {
        clearInterval(pingIntervalRef.current)
        pingIntervalRef.current = null
      }
    })

    socketRef.current.on('connect_error', (error: Error) => {
      console.error('[WS] Connection error:', error.message)
    })

    socketRef.current.on('error', (error: Error) => {
      console.error('[WS] Error:', error)
    })

    // Generic event handler
    socketRef.current.onAny((event: string, payload: unknown) => {
      const listeners = listenersRef.current.get(event)
      if (listeners) {
        listeners.forEach(callback => callback(payload))
      }
    })
  }, [reconnect])

  const disconnect = useCallback(() => {
    if (pingIntervalRef.current) {
      clearInterval(pingIntervalRef.current)
      pingIntervalRef.current = null
    }
    socketRef.current?.disconnect()
    socketRef.current = null
    setIsConnected(false)
    setLatency(0)
  }, [])

  const subscribe = useCallback(<T = unknown>(event: RealtimeEventType, callback: (payload: T) => void) => {
    if (!listenersRef.current.has(event)) {
      listenersRef.current.set(event, new Set())
    }
    listenersRef.current.get(event)!.add(callback as (payload: unknown) => void)

    // Return unsubscribe function
    return () => {
      listenersRef.current.get(event)?.delete(callback as (payload: unknown) => void)
    }
  }, [])

  const emit = useCallback((event: string, data: unknown) => {
    socketRef.current?.emit(event, data)
  }, [])

  const joinRoom = useCallback((room: string) => {
    socketRef.current?.emit('room.join', { room })
  }, [])

  const leaveRoom = useCallback((room: string) => {
    socketRef.current?.emit('room.leave', { room })
  }, [])

  // Auto-connect on mount
  useEffect(() => {
    if (autoConnect) {
      connect()
    }
    return () => {
      disconnect()
    }
  }, [autoConnect, connect, disconnect])

  return {
    isConnected,
    latency,
    connect,
    disconnect,
    subscribe,
    emit,
    joinRoom,
    leaveRoom,
  }
}

// ====== Convenience Hooks ======

// Online players tracking
export function useOnlinePlayersCount() {
  const [count, setCount] = useState(0)
  const { subscribe, isConnected } = useRealtime()

  useEffect(() => {
    const unsubscribe = subscribe<PlayerLoginPayload>('player.online', () => setCount(c => c + 1))
    const unsubscribe2 = subscribe<PlayerLogoutPayload>('player.offline', () => setCount(c => Math.max(0, c - 1)))
    
    return () => {
      unsubscribe()
      unsubscribe2()
    }
  }, [subscribe])

  return { count, isConnected }
}

// Server broadcast messages
export function useServerBroadcast(callback: (message: string) => void) {
  const { subscribe } = useRealtime()

  useEffect(() => {
    return subscribe<{ message: string }>('server.broadcast', (payload) => {
      callback(payload.message)
    })
  }, [subscribe, callback])
}

// Player events feed (level ups, deaths, achievements)
export function usePlayerEvents(maxEvents = 50) {
  const [events, setEvents] = useState<RealtimeEvent[]>([])
  const { subscribe } = useRealtime()

  useEffect(() => {
    const eventTypes: RealtimeEventType[] = [
      'player.levelup',
      'player.death',
      'player.achievement',
    ]

    const unsubscribes = eventTypes.map(type => 
      subscribe(type, (payload) => {
        setEvents(prev => [{
          type,
          payload,
          timestamp: Date.now()
        }, ...prev.slice(0, maxEvents - 1)])
      })
    )

    return () => unsubscribes.forEach(unsub => unsub())
  }, [subscribe, maxEvents])

  return events
}

// Level up notifications
export function useLevelUpFeed(maxItems = 10) {
  const [levelUps, setLevelUps] = useState<(LevelUpPayload & { timestamp: number })[]>([])
  const { subscribe } = useRealtime()

  useEffect(() => {
    return subscribe<LevelUpPayload>('player.levelup', (payload) => {
      setLevelUps(prev => [
        { ...payload, timestamp: Date.now() },
        ...prev.slice(0, maxItems - 1)
      ])
    })
  }, [subscribe, maxItems])

  return levelUps
}

// Death notifications
export function useDeathFeed(maxItems = 10) {
  const [deaths, setDeaths] = useState<(DeathPayload & { timestamp: number })[]>([])
  const { subscribe } = useRealtime()

  useEffect(() => {
    return subscribe<DeathPayload>('player.death', (payload) => {
      setDeaths(prev => [
        { ...payload, timestamp: Date.now() },
        ...prev.slice(0, maxItems - 1)
      ])
    })
  }, [subscribe, maxItems])

  return deaths
}

// Achievement notifications
export function useAchievementFeed(maxItems = 10) {
  const [achievements, setAchievements] = useState<(AchievementPayload & { timestamp: number })[]>([])
  const { subscribe } = useRealtime()

  useEffect(() => {
    return subscribe<AchievementPayload>('player.achievement', (payload) => {
      setAchievements(prev => [
        { ...payload, timestamp: Date.now() },
        ...prev.slice(0, maxItems - 1)
      ])
    })
  }, [subscribe, maxItems])

  return achievements
}

// Real-time realm status
export function useRealmStatus(realmId?: string) {
  const [realms, setRealms] = useState<Map<string, RealmStatusPayload>>(new Map())
  const { subscribe } = useRealtime()

  useEffect(() => {
    return subscribe<RealmStatusPayload>('realm.status', (payload) => {
      if (!realmId || payload.realmId === realmId) {
        setRealms(prev => {
          const next = new Map(prev)
          next.set(payload.realmId, payload)
          return next
        })
      }
    })
  }, [subscribe, realmId])

  return realmId ? realms.get(realmId) : Array.from(realms.values())
}

// Live chat room
export function useChatRoom(room: string) {
  const [messages, setMessages] = useState<ChatMessagePayload[]>([])
  const { subscribe, emit, joinRoom, leaveRoom, isConnected } = useRealtime()

  useEffect(() => {
    if (isConnected) {
      joinRoom(room)
    }
    return () => {
      if (isConnected) {
        leaveRoom(room)
      }
    }
  }, [room, joinRoom, leaveRoom, isConnected])

  useEffect(() => {
    return subscribe<ChatMessagePayload>('chat.message', (payload) => {
      if (payload.room === room) {
        setMessages(prev => [...prev, payload].slice(-100)) // Keep last 100 messages
      }
    })
  }, [subscribe, room])

  const sendMessage = useCallback((message: string) => {
    emit('chat.send', { room, message })
  }, [emit, room])

  return { messages, sendMessage, isConnected }
}

// Market price updates
export function useMarketUpdates(itemIds?: number[]) {
  const [prices, setPrices] = useState<Map<number, MarketUpdatePayload>>(new Map())
  const { subscribe } = useRealtime()

  useEffect(() => {
    return subscribe<MarketUpdatePayload>('market.update', (payload) => {
      if (!itemIds || itemIds.includes(payload.itemId)) {
        setPrices(prev => {
          const next = new Map(prev)
          next.set(payload.itemId, payload)
          return next
        })
      }
    })
  }, [subscribe, itemIds])

  return prices
}

// Guild war updates
export function useGuildWar(guildId?: number) {
  const [wars, setWars] = useState<GuildWarPayload[]>([])
  const { subscribe } = useRealtime()

  useEffect(() => {
    return subscribe<GuildWarPayload>('guild.war', (payload) => {
      if (!guildId || payload.guildA.id === guildId || payload.guildB.id === guildId) {
        setWars(prev => {
          // Update existing war or add new one
          const existing = prev.findIndex(
            w => (w.guildA.id === payload.guildA.id && w.guildB.id === payload.guildB.id)
          )
          if (existing >= 0) {
            const next = [...prev]
            next[existing] = payload
            return next
          }
          return [payload, ...prev]
        })
      }
    })
  }, [subscribe, guildId])

  return wars
}

// Highscore live updates
export function useHighscoreLive(category: string, limit = 10) {
  const [highscores, setHighscores] = useState<Array<{
    rank: number
    characterId: number
    characterName: string
    value: number
    change?: number
  }>>([])
  const { subscribe, joinRoom, leaveRoom, isConnected } = useRealtime()

  useEffect(() => {
    const room = `highscore.${category}`
    if (isConnected) {
      joinRoom(room)
    }
    return () => {
      if (isConnected) {
        leaveRoom(room)
      }
    }
  }, [category, joinRoom, leaveRoom, isConnected])

  useEffect(() => {
    return subscribe<typeof highscores>(`highscore.${category}` as RealtimeEventType, (payload) => {
      setHighscores(payload.slice(0, limit))
    })
  }, [subscribe, category, limit])

  return highscores
}

// Live server stats
export function useServerStats() {
  const [stats, setStats] = useState<{
    playersOnline: number
    peak24h: number
    monstersKilled: number
    itemsTraded: number
    goldCirculating: number
  } | null>(null)
  const { subscribe } = useRealtime()

  useEffect(() => {
    return subscribe('server.stats', (payload) => {
      setStats(payload as typeof stats)
    })
  }, [subscribe])

  return stats
}

// Connection status indicator component hook
export function useConnectionStatus() {
  const { isConnected, latency } = useRealtime()
  
  const status = isConnected
    ? latency < 100 ? 'excellent' : latency < 200 ? 'good' : 'poor'
    : 'disconnected'
  
  const color = {
    excellent: '#22c55e',
    good: '#eab308',
    poor: '#f97316',
    disconnected: '#ef4444',
  }[status]

  return { isConnected, latency, status, color }
}

