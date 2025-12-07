import { create } from 'zustand'
import { OTBMMap, OTBMTile, OTBMItem, getTileKey, createEmptyMap, parseOTBM, serializeOTBM } from './otbm'

export type Tool = 'select' | 'brush' | 'fill' | 'erase' | 'picker' | 'spawn' | 'waypoint' | 'zone'

export interface Selection {
  startX: number
  startY: number
  endX: number
  endY: number
  z: number
}

export interface Brush {
  tileId: number
  size: number
  shape: 'square' | 'circle' | 'diamond'
}

export interface HistoryEntry {
  tiles: Map<string, OTBMTile | null>
  description: string
}

interface MapState {
  // Map data
  map: OTBMMap | null
  fileName: string | null
  isDirty: boolean
  
  // Viewport
  viewportX: number
  viewportY: number
  zoom: number
  currentFloor: number
  
  // Tools
  activeTool: Tool
  brush: Brush
  
  // Selection
  selection: Selection | null
  hoveredTile: { x: number; y: number; z: number } | null
  
  // Layers
  showGround: boolean
  showItems: boolean
  showCreatures: boolean
  showZones: boolean
  showGrid: boolean
  
  // History
  history: HistoryEntry[]
  historyIndex: number
  
  // Actions
  newMap: (width: number, height: number) => void
  loadMap: (buffer: ArrayBuffer, fileName: string) => Promise<void>
  saveMap: () => ArrayBuffer | null
  
  setViewport: (x: number, y: number) => void
  setZoom: (zoom: number) => void
  setFloor: (floor: number) => void
  
  setTool: (tool: Tool) => void
  setBrush: (brush: Partial<Brush>) => void
  
  setSelection: (selection: Selection | null) => void
  setHoveredTile: (tile: { x: number; y: number; z: number } | null) => void
  
  toggleLayer: (layer: 'ground' | 'items' | 'creatures' | 'zones' | 'grid') => void
  
  // Tile operations
  placeTile: (x: number, y: number, z: number) => void
  eraseTile: (x: number, y: number, z: number) => void
  fillArea: (startX: number, startY: number, endX: number, endY: number, z: number) => void
  copySelection: () => Map<string, OTBMTile> | null
  pasteSelection: (tiles: Map<string, OTBMTile>, offsetX: number, offsetY: number) => void
  
  // History
  undo: () => void
  redo: () => void
  pushHistory: (tiles: Map<string, OTBMTile | null>, description: string) => void
}

export const useMapStore = create<MapState>((set, get) => ({
  // Initial state
  map: null,
  fileName: null,
  isDirty: false,
  
  viewportX: 0,
  viewportY: 0,
  zoom: 1,
  currentFloor: 7,
  
  activeTool: 'brush',
  brush: {
    tileId: 4526, // Default grass tile
    size: 1,
    shape: 'square',
  },
  
  selection: null,
  hoveredTile: null,
  
  showGround: true,
  showItems: true,
  showCreatures: true,
  showZones: true,
  showGrid: true,
  
  history: [],
  historyIndex: -1,
  
  // Actions
  newMap: (width: number, height: number) => {
    const map = createEmptyMap(width, height)
    set({
      map,
      fileName: null,
      isDirty: false,
      viewportX: Math.floor(width / 2),
      viewportY: Math.floor(height / 2),
      history: [],
      historyIndex: -1,
    })
  },
  
  loadMap: async (buffer: ArrayBuffer, fileName: string) => {
    try {
      const map = await parseOTBM(buffer)
      set({
        map,
        fileName,
        isDirty: false,
        history: [],
        historyIndex: -1,
      })
    } catch (error) {
      console.error('Failed to load map:', error)
      throw error
    }
  },
  
  saveMap: () => {
    const { map } = get()
    if (!map) return null
    
    const buffer = serializeOTBM(map)
    set({ isDirty: false })
    return buffer
  },
  
  setViewport: (x: number, y: number) => {
    set({ viewportX: x, viewportY: y })
  },
  
  setZoom: (zoom: number) => {
    set({ zoom: Math.max(0.25, Math.min(4, zoom)) })
  },
  
  setFloor: (floor: number) => {
    set({ currentFloor: Math.max(0, Math.min(15, floor)) })
  },
  
  setTool: (tool: Tool) => {
    set({ activeTool: tool })
  },
  
  setBrush: (brush: Partial<Brush>) => {
    set(state => ({
      brush: { ...state.brush, ...brush }
    }))
  },
  
  setSelection: (selection: Selection | null) => {
    set({ selection })
  },
  
  setHoveredTile: (tile: { x: number; y: number; z: number } | null) => {
    set({ hoveredTile: tile })
  },
  
  toggleLayer: (layer: 'ground' | 'items' | 'creatures' | 'zones' | 'grid') => {
    switch (layer) {
      case 'ground':
        set(state => ({ showGround: !state.showGround }))
        break
      case 'items':
        set(state => ({ showItems: !state.showItems }))
        break
      case 'creatures':
        set(state => ({ showCreatures: !state.showCreatures }))
        break
      case 'zones':
        set(state => ({ showZones: !state.showZones }))
        break
      case 'grid':
        set(state => ({ showGrid: !state.showGrid }))
        break
    }
  },
  
  placeTile: (x: number, y: number, z: number) => {
    const { map, brush, pushHistory } = get()
    if (!map) return
    
    const key = getTileKey(x, y, z)
    const existingTile = map.tiles.get(key)
    
    // Store old tile for history
    const oldTiles = new Map<string, OTBMTile | null>()
    oldTiles.set(key, existingTile ? { ...existingTile, items: [...existingTile.items] } : null)
    
    // Create or update tile
    const newTile: OTBMTile = {
      x,
      y,
      z,
      tileId: brush.tileId,
      items: existingTile?.items || [],
    }
    
    map.tiles.set(key, newTile)
    set({ map: { ...map }, isDirty: true })
    pushHistory(oldTiles, 'Place tile')
  },
  
  eraseTile: (x: number, y: number, z: number) => {
    const { map, pushHistory } = get()
    if (!map) return
    
    const key = getTileKey(x, y, z)
    const existingTile = map.tiles.get(key)
    
    if (!existingTile) return
    
    // Store old tile for history
    const oldTiles = new Map<string, OTBMTile | null>()
    oldTiles.set(key, { ...existingTile, items: [...existingTile.items] })
    
    map.tiles.delete(key)
    set({ map: { ...map }, isDirty: true })
    pushHistory(oldTiles, 'Erase tile')
  },
  
  fillArea: (startX: number, startY: number, endX: number, endY: number, z: number) => {
    const { map, brush, pushHistory } = get()
    if (!map) return
    
    const minX = Math.min(startX, endX)
    const maxX = Math.max(startX, endX)
    const minY = Math.min(startY, endY)
    const maxY = Math.max(startY, endY)
    
    const oldTiles = new Map<string, OTBMTile | null>()
    
    for (let x = minX; x <= maxX; x++) {
      for (let y = minY; y <= maxY; y++) {
        const key = getTileKey(x, y, z)
        const existingTile = map.tiles.get(key)
        
        oldTiles.set(key, existingTile ? { ...existingTile, items: [...existingTile.items] } : null)
        
        const newTile: OTBMTile = {
          x,
          y,
          z,
          tileId: brush.tileId,
          items: existingTile?.items || [],
        }
        
        map.tiles.set(key, newTile)
      }
    }
    
    set({ map: { ...map }, isDirty: true })
    pushHistory(oldTiles, `Fill area (${maxX - minX + 1}x${maxY - minY + 1})`)
  },
  
  copySelection: () => {
    const { map, selection } = get()
    if (!map || !selection) return null
    
    const tiles = new Map<string, OTBMTile>()
    const minX = Math.min(selection.startX, selection.endX)
    const maxX = Math.max(selection.startX, selection.endX)
    const minY = Math.min(selection.startY, selection.endY)
    const maxY = Math.max(selection.startY, selection.endY)
    
    for (let x = minX; x <= maxX; x++) {
      for (let y = minY; y <= maxY; y++) {
        const key = getTileKey(x, y, selection.z)
        const tile = map.tiles.get(key)
        
        if (tile) {
          // Store with relative positions
          const relativeKey = getTileKey(x - minX, y - minY, 0)
          tiles.set(relativeKey, { ...tile, items: [...tile.items] })
        }
      }
    }
    
    return tiles
  },
  
  pasteSelection: (tiles: Map<string, OTBMTile>, offsetX: number, offsetY: number) => {
    const { map, currentFloor, pushHistory } = get()
    if (!map) return
    
    const oldTiles = new Map<string, OTBMTile | null>()
    
    for (const [relKey, tile] of tiles) {
      const { x: relX, y: relY } = { x: parseInt(relKey.split(':')[0]), y: parseInt(relKey.split(':')[1]) }
      const newX = offsetX + relX
      const newY = offsetY + relY
      const key = getTileKey(newX, newY, currentFloor)
      
      const existingTile = map.tiles.get(key)
      oldTiles.set(key, existingTile ? { ...existingTile, items: [...existingTile.items] } : null)
      
      const newTile: OTBMTile = {
        ...tile,
        x: newX,
        y: newY,
        z: currentFloor,
        items: [...tile.items],
      }
      
      map.tiles.set(key, newTile)
    }
    
    set({ map: { ...map }, isDirty: true })
    pushHistory(oldTiles, `Paste (${tiles.size} tiles)`)
  },
  
  undo: () => {
    const { map, history, historyIndex } = get()
    if (!map || historyIndex < 0) return
    
    const entry = history[historyIndex]
    
    // Restore old tiles
    for (const [key, oldTile] of entry.tiles) {
      if (oldTile === null) {
        map.tiles.delete(key)
      } else {
        map.tiles.set(key, oldTile)
      }
    }
    
    set({
      map: { ...map },
      historyIndex: historyIndex - 1,
      isDirty: true,
    })
  },
  
  redo: () => {
    const { map, history, historyIndex } = get()
    if (!map || historyIndex >= history.length - 1) return
    
    const nextEntry = history[historyIndex + 1]
    
    // This is a simplified redo - in a real implementation,
    // we'd need to store the "after" state as well
    set({
      historyIndex: historyIndex + 1,
      isDirty: true,
    })
  },
  
  pushHistory: (tiles: Map<string, OTBMTile | null>, description: string) => {
    const { history, historyIndex } = get()
    
    // Remove any redo history
    const newHistory = history.slice(0, historyIndex + 1)
    
    newHistory.push({ tiles, description })
    
    // Limit history size
    if (newHistory.length > 100) {
      newHistory.shift()
    }
    
    set({
      history: newHistory,
      historyIndex: newHistory.length - 1,
    })
  },
}))


