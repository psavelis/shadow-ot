'use client'

import { useCallback, useRef, useEffect } from 'react'
import { motion } from 'framer-motion'
import { 
  MousePointer2, 
  Pencil, 
  Eraser, 
  PaintBucket, 
  Pipette,
  Move,
  ZoomIn,
  ZoomOut,
  Undo,
  Redo,
  Save,
  FolderOpen,
  Download,
  Upload,
  Eye,
  EyeOff,
  Settings,
  Grid3X3,
  Home,
  Plus,
  Minus,
  RotateCcw,
  Copy,
  Trash2,
  FileText,
  MapPin,
  Shield,
  AlertTriangle
} from 'lucide-react'
import * as Tooltip from '@radix-ui/react-tooltip'
import * as Tabs from '@radix-ui/react-tabs'
import * as Dialog from '@radix-ui/react-dialog'
import { MapCanvas } from '@/components/MapCanvas'
import { useMapStore, Tool } from '@/lib/mapStore'

const tools: { id: Tool; name: string; icon: typeof MousePointer2; shortcut: string }[] = [
  { id: 'select', name: 'Select', icon: MousePointer2, shortcut: 'V' },
  { id: 'brush', name: 'Brush', icon: Pencil, shortcut: 'B' },
  { id: 'erase', name: 'Eraser', icon: Eraser, shortcut: 'E' },
  { id: 'fill', name: 'Fill', icon: PaintBucket, shortcut: 'G' },
  { id: 'picker', name: 'Color Picker', icon: Pipette, shortcut: 'I' },
  { id: 'spawn', name: 'Spawn Point', icon: MapPin, shortcut: 'S' },
  { id: 'zone', name: 'Zone', icon: Shield, shortcut: 'Z' },
]

const tileCategories = [
  { 
    id: 'ground', 
    name: 'Ground Tiles',
    tiles: [
      { id: 4526, name: 'Grass', color: '#2d5a27' },
      { id: 4527, name: 'Dark Grass', color: '#1a4d1a' },
      { id: 4405, name: 'Dirt', color: '#8b7355' },
      { id: 4406, name: 'Dark Dirt', color: '#6b5344' },
      { id: 4907, name: 'Sand', color: '#e6cc80' },
      { id: 4908, name: 'Dark Sand', color: '#d4b366' },
    ]
  },
  {
    id: 'terrain',
    name: 'Terrain',
    tiles: [
      { id: 4835, name: 'Stone', color: '#cccccc' },
      { id: 4836, name: 'Dark Stone', color: '#999999' },
      { id: 4820, name: 'Water', color: '#3366cc' },
      { id: 4821, name: 'Deep Water', color: '#2255bb' },
      { id: 105, name: 'Void', color: '#111111' },
    ]
  },
]

export default function MapMakerPage() {
  const fileInputRef = useRef<HTMLInputElement>(null)
  
  const {
    map,
    fileName,
    isDirty,
    viewportX,
    viewportY,
    zoom,
    currentFloor,
    activeTool,
    brush,
    selection,
    hoveredTile,
    showGround,
    showItems,
    showCreatures,
    showZones,
    showGrid,
    history,
    historyIndex,
    newMap,
    loadMap,
    saveMap,
    setViewport,
    setZoom,
    setFloor,
    setTool,
    setBrush,
    setSelection,
    toggleLayer,
    undo,
    redo,
    copySelection,
  } = useMapStore()

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
        return
      }

      // Tool shortcuts
      if (!e.ctrlKey && !e.metaKey) {
        const tool = tools.find(t => t.shortcut.toLowerCase() === e.key.toLowerCase())
        if (tool) {
          setTool(tool.id)
          return
        }
      }

      // Ctrl/Cmd shortcuts
      if (e.ctrlKey || e.metaKey) {
        switch (e.key.toLowerCase()) {
          case 'z':
            e.preventDefault()
            if (e.shiftKey) {
              redo()
            } else {
              undo()
            }
            break
          case 'y':
            e.preventDefault()
            redo()
            break
          case 's':
            e.preventDefault()
            handleSave()
            break
          case 'o':
            e.preventDefault()
            fileInputRef.current?.click()
            break
          case 'c':
            if (selection) {
              e.preventDefault()
              copySelection()
            }
            break
        }
      }

      // Floor navigation
      if (e.key === 'PageUp') {
        setFloor(currentFloor - 1)
      } else if (e.key === 'PageDown') {
        setFloor(currentFloor + 1)
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [setTool, undo, redo, selection, copySelection, currentFloor, setFloor])

  const handleNewMap = () => {
    newMap(256, 256)
  }

  const handleOpenFile = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (!file) return

    try {
      const buffer = await file.arrayBuffer()
      await loadMap(buffer, file.name)
    } catch (error) {
      console.error('Failed to load map:', error)
      alert('Failed to load map file. Make sure it\'s a valid OTBM file.')
    }
  }

  const handleSave = () => {
    const buffer = saveMap()
    if (!buffer) {
      alert('No map to save')
      return
    }

    const blob = new Blob([buffer], { type: 'application/octet-stream' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = fileName || 'map.otbm'
    a.click()
    URL.revokeObjectURL(url)
  }

  const canUndo = historyIndex >= 0
  const canRedo = historyIndex < history.length - 1

  return (
    <Tooltip.Provider delayDuration={300}>
      <div className="h-screen flex flex-col bg-slate-950">
        {/* Hidden file input */}
        <input
          ref={fileInputRef}
          type="file"
          accept=".otbm"
          onChange={handleOpenFile}
          className="hidden"
        />

        {/* Top Bar */}
        <header className="h-12 bg-slate-900/90 border-b border-slate-700 flex items-center justify-between px-4">
          <div className="flex items-center space-x-4">
            <div className="flex items-center space-x-2">
              <div className="w-7 h-7 bg-gradient-to-br from-amber-500 to-orange-600 rounded flex items-center justify-center">
                <Grid3X3 className="w-4 h-4 text-white" />
              </div>
              <span className="font-bold text-sm text-white">Shadow Map Maker</span>
            </div>
            <div className="h-6 w-px bg-slate-700" />
            
            {/* File operations */}
            <div className="flex items-center space-x-1">
              <Tooltip.Root>
                <Tooltip.Trigger asChild>
                  <button onClick={handleNewMap} className="p-2 hover:bg-slate-800 rounded transition text-slate-400 hover:text-white">
                    <FileText className="w-4 h-4" />
                  </button>
                </Tooltip.Trigger>
                <Tooltip.Content className="bg-slate-800 text-white text-xs px-2 py-1 rounded" side="bottom">
                  New Map (Ctrl+N)
                </Tooltip.Content>
              </Tooltip.Root>
              
              <Tooltip.Root>
                <Tooltip.Trigger asChild>
                  <button onClick={() => fileInputRef.current?.click()} className="p-2 hover:bg-slate-800 rounded transition text-slate-400 hover:text-white">
                    <FolderOpen className="w-4 h-4" />
                  </button>
                </Tooltip.Trigger>
                <Tooltip.Content className="bg-slate-800 text-white text-xs px-2 py-1 rounded" side="bottom">
                  Open (Ctrl+O)
                </Tooltip.Content>
              </Tooltip.Root>
              
              <Tooltip.Root>
                <Tooltip.Trigger asChild>
                  <button onClick={handleSave} className="p-2 hover:bg-slate-800 rounded transition text-slate-400 hover:text-white">
                    <Save className="w-4 h-4" />
                  </button>
                </Tooltip.Trigger>
                <Tooltip.Content className="bg-slate-800 text-white text-xs px-2 py-1 rounded" side="bottom">
                  Save (Ctrl+S)
                </Tooltip.Content>
              </Tooltip.Root>
            </div>
            
            <div className="h-6 w-px bg-slate-700" />
            
            {/* Undo/Redo */}
            <div className="flex items-center space-x-1">
              <Tooltip.Root>
                <Tooltip.Trigger asChild>
                  <button 
                    onClick={undo} 
                    disabled={!canUndo}
                    className={`p-2 hover:bg-slate-800 rounded transition ${canUndo ? 'text-slate-400 hover:text-white' : 'text-slate-600 cursor-not-allowed'}`}
                  >
                    <Undo className="w-4 h-4" />
                  </button>
                </Tooltip.Trigger>
                <Tooltip.Content className="bg-slate-800 text-white text-xs px-2 py-1 rounded" side="bottom">
                  Undo (Ctrl+Z)
                </Tooltip.Content>
              </Tooltip.Root>
              
              <Tooltip.Root>
                <Tooltip.Trigger asChild>
                  <button 
                    onClick={redo}
                    disabled={!canRedo}
                    className={`p-2 hover:bg-slate-800 rounded transition ${canRedo ? 'text-slate-400 hover:text-white' : 'text-slate-600 cursor-not-allowed'}`}
                  >
                    <Redo className="w-4 h-4" />
                  </button>
                </Tooltip.Trigger>
                <Tooltip.Content className="bg-slate-800 text-white text-xs px-2 py-1 rounded" side="bottom">
                  Redo (Ctrl+Y)
                </Tooltip.Content>
              </Tooltip.Root>
            </div>
          </div>
          
          <div className="flex items-center space-x-4">
            {/* Position display */}
            <div className="flex items-center space-x-2 bg-slate-800 rounded-lg px-3 py-1.5">
              <span className="text-slate-500 text-xs">Position:</span>
              <span className="font-mono text-xs text-white">
                {hoveredTile ? `${hoveredTile.x}, ${hoveredTile.y}, ${currentFloor}` : `${viewportX}, ${viewportY}, ${currentFloor}`}
              </span>
            </div>
            
            {/* Zoom controls */}
            <div className="flex items-center space-x-2">
              <Tooltip.Root>
                <Tooltip.Trigger asChild>
                  <button 
                    onClick={() => setZoom(zoom - 0.25)} 
                    className="p-2 hover:bg-slate-800 rounded transition text-slate-400 hover:text-white"
                  >
                    <ZoomOut className="w-4 h-4" />
                  </button>
                </Tooltip.Trigger>
                <Tooltip.Content className="bg-slate-800 text-white text-xs px-2 py-1 rounded" side="bottom">
                  Zoom Out
                </Tooltip.Content>
              </Tooltip.Root>
              
              <span className="text-xs text-slate-300 w-12 text-center">{Math.round(zoom * 100)}%</span>
              
              <Tooltip.Root>
                <Tooltip.Trigger asChild>
                  <button 
                    onClick={() => setZoom(zoom + 0.25)} 
                    className="p-2 hover:bg-slate-800 rounded transition text-slate-400 hover:text-white"
                  >
                    <ZoomIn className="w-4 h-4" />
                  </button>
                </Tooltip.Trigger>
                <Tooltip.Content className="bg-slate-800 text-white text-xs px-2 py-1 rounded" side="bottom">
                  Zoom In
                </Tooltip.Content>
              </Tooltip.Root>
            </div>
            
            <button className="p-2 hover:bg-slate-800 rounded transition text-slate-400 hover:text-white">
              <Settings className="w-4 h-4" />
            </button>
          </div>
        </header>

        <div className="flex-1 flex overflow-hidden">
          {/* Left Toolbar */}
          <aside className="w-12 bg-slate-900/90 border-r border-slate-700 flex flex-col items-center py-2 space-y-1">
            {tools.map((tool) => (
              <Tooltip.Root key={tool.id}>
                <Tooltip.Trigger asChild>
                  <button
                    onClick={() => setTool(tool.id)}
                    className={`w-9 h-9 flex items-center justify-center rounded transition ${
                      activeTool === tool.id 
                        ? 'bg-amber-500/20 text-amber-400' 
                        : 'text-slate-400 hover:bg-slate-800 hover:text-white'
                    }`}
                  >
                    <tool.icon className="w-5 h-5" />
                  </button>
                </Tooltip.Trigger>
                <Tooltip.Portal>
                  <Tooltip.Content className="bg-slate-800 text-white text-xs px-2 py-1 rounded" side="right" sideOffset={8}>
                    {tool.name} ({tool.shortcut})
                  </Tooltip.Content>
                </Tooltip.Portal>
              </Tooltip.Root>
            ))}
            
            <div className="flex-1" />
            
            {/* Brush size */}
            <div className="flex flex-col items-center space-y-1 pb-2">
              <span className="text-[10px] text-slate-500">Size</span>
              <div className="flex flex-col space-y-1">
                <button 
                  onClick={() => setBrush({ size: Math.min(10, brush.size + 1) })}
                  className="p-1 hover:bg-slate-800 rounded text-slate-400 hover:text-white"
                >
                  <Plus className="w-3 h-3" />
                </button>
                <span className="text-xs text-white text-center">{brush.size}</span>
                <button 
                  onClick={() => setBrush({ size: Math.max(1, brush.size - 1) })}
                  className="p-1 hover:bg-slate-800 rounded text-slate-400 hover:text-white"
                >
                  <Minus className="w-3 h-3" />
                </button>
              </div>
            </div>
          </aside>

          {/* Main Canvas Area */}
          <main className="flex-1 relative overflow-hidden">
            <MapCanvas />

            {/* Floor Selector */}
            <div className="absolute bottom-4 left-4 bg-slate-800/90 border border-slate-700 rounded-lg p-2">
              <div className="flex items-center space-x-2">
                <button 
                  onClick={() => setFloor(currentFloor + 1)} 
                  className="p-1.5 hover:bg-slate-700 rounded text-slate-400 hover:text-white transition"
                >
                  <Plus className="w-4 h-4" />
                </button>
                <div className="w-16 text-center">
                  <span className="text-[10px] text-slate-500 block">Floor</span>
                  <p className="font-mono text-sm text-white">{currentFloor}</p>
                </div>
                <button 
                  onClick={() => setFloor(currentFloor - 1)} 
                  className="p-1.5 hover:bg-slate-700 rounded text-slate-400 hover:text-white transition"
                >
                  <Minus className="w-4 h-4" />
                </button>
              </div>
            </div>

            {/* Minimap */}
            <div className="absolute top-4 right-4 bg-slate-800/90 border border-slate-700 rounded-lg p-2 w-40">
              <div className="aspect-square bg-slate-900 rounded border border-slate-600 mb-2 relative overflow-hidden">
                <div className="absolute inset-0 bg-gradient-to-br from-green-900/30 to-blue-900/30" />
                <div 
                  className="absolute w-2 h-2 bg-amber-500 rounded-sm" 
                  style={{ left: '50%', top: '50%', transform: 'translate(-50%, -50%)' }} 
                />
              </div>
              <div className="flex items-center justify-between">
                <button 
                  onClick={() => setViewport(map?.header.width ? map.header.width / 2 : 128, map?.header.height ? map.header.height / 2 : 128)} 
                  className="p-1.5 hover:bg-slate-700 rounded text-slate-400 hover:text-white transition"
                >
                  <Home className="w-3 h-3" />
                </button>
                <button 
                  onClick={() => setZoom(1)} 
                  className="p-1.5 hover:bg-slate-700 rounded text-slate-400 hover:text-white transition"
                >
                  <RotateCcw className="w-3 h-3" />
                </button>
              </div>
            </div>

            {/* No map message overlay */}
            {!map && (
              <div className="absolute inset-0 flex items-center justify-center pointer-events-none">
                <motion.div 
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  className="text-center pointer-events-auto"
                >
                  <div className="w-20 h-20 bg-slate-800/50 rounded-full flex items-center justify-center mx-auto mb-6">
                    <Grid3X3 className="w-10 h-10 text-slate-600" />
                  </div>
                  <h2 className="text-xl font-medium text-white mb-2">No Map Loaded</h2>
                  <p className="text-slate-400 max-w-xs mx-auto mb-6">
                    Create a new map or open an existing OTBM file to get started.
                  </p>
                  <div className="flex items-center gap-3 justify-center">
                    <button 
                      onClick={handleNewMap}
                      className="px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-600 text-white rounded-lg hover:from-amber-400 hover:to-orange-500 transition text-sm"
                    >
                      New Map
                    </button>
                    <button 
                      onClick={() => fileInputRef.current?.click()}
                      className="px-4 py-2 bg-slate-800 text-white rounded-lg hover:bg-slate-700 transition text-sm"
                    >
                      Open File
                    </button>
                  </div>
                </motion.div>
              </div>
            )}
          </main>

          {/* Right Panel */}
          <aside className="w-64 bg-slate-900/90 border-l border-slate-700 flex flex-col">
            <Tabs.Root defaultValue="tiles" className="flex-1 flex flex-col">
              <Tabs.List className="flex border-b border-slate-700">
                <Tabs.Trigger value="tiles" className="flex-1 py-2 text-xs font-medium text-slate-400 data-[state=active]:text-white data-[state=active]:border-b-2 data-[state=active]:border-amber-500 transition">
                  Tiles
                </Tabs.Trigger>
                <Tabs.Trigger value="layers" className="flex-1 py-2 text-xs font-medium text-slate-400 data-[state=active]:text-white data-[state=active]:border-b-2 data-[state=active]:border-amber-500 transition">
                  Layers
                </Tabs.Trigger>
                <Tabs.Trigger value="properties" className="flex-1 py-2 text-xs font-medium text-slate-400 data-[state=active]:text-white data-[state=active]:border-b-2 data-[state=active]:border-amber-500 transition">
                  Props
                </Tabs.Trigger>
              </Tabs.List>

              <Tabs.Content value="tiles" className="flex-1 overflow-y-auto p-3">
                <input 
                  type="text" 
                  placeholder="Search tiles..." 
                  className="w-full bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 text-sm text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50 mb-3" 
                />
                <div className="space-y-3">
                  {tileCategories.map((category) => (
                    <div key={category.id}>
                      <h4 className="text-xs font-medium text-slate-500 uppercase mb-2">{category.name}</h4>
                      <div className="grid grid-cols-4 gap-1">
                        {category.tiles.map((tile) => (
                          <Tooltip.Root key={tile.id}>
                            <Tooltip.Trigger asChild>
                              <button
                                onClick={() => setBrush({ tileId: tile.id })}
                                className={`aspect-square rounded border-2 transition ${
                                  brush.tileId === tile.id 
                                    ? 'border-amber-500' 
                                    : 'border-transparent hover:border-slate-600'
                                }`}
                                style={{ backgroundColor: tile.color }}
                              />
                            </Tooltip.Trigger>
                            <Tooltip.Content className="bg-slate-800 text-white text-xs px-2 py-1 rounded" side="left">
                              {tile.name} (#{tile.id})
                            </Tooltip.Content>
                          </Tooltip.Root>
                        ))}
                        </div>
                    </div>
                  ))}
                </div>
              </Tabs.Content>

              <Tabs.Content value="layers" className="flex-1 overflow-y-auto p-3">
                <div className="space-y-2">
                  {[
                    { id: 'ground' as const, name: 'Ground', visible: showGround },
                    { id: 'items' as const, name: 'Items', visible: showItems },
                    { id: 'creatures' as const, name: 'Creatures', visible: showCreatures },
                    { id: 'zones' as const, name: 'Zones', visible: showZones },
                    { id: 'grid' as const, name: 'Grid', visible: showGrid },
                  ].map((layer) => (
                    <div key={layer.id} className="flex items-center justify-between px-3 py-2 bg-slate-800/50 rounded-lg">
                      <span className="text-sm text-white">{layer.name}</span>
                      <button 
                        onClick={() => toggleLayer(layer.id)}
                        className={`p-1 rounded transition ${layer.visible ? 'text-amber-400' : 'text-slate-500'}`}
                      >
                        {layer.visible ? <Eye className="w-4 h-4" /> : <EyeOff className="w-4 h-4" />}
                      </button>
                    </div>
                  ))}
                </div>
              </Tabs.Content>

              <Tabs.Content value="properties" className="flex-1 overflow-y-auto p-3">
                {selection ? (
                  <div className="space-y-3">
                    <h4 className="text-xs font-medium text-slate-500 uppercase">Selection</h4>
                    <div className="bg-slate-800/50 rounded-lg p-3 space-y-2 text-sm">
                      <div className="flex justify-between">
                        <span className="text-slate-400">Start</span>
                        <span className="text-white font-mono">{selection.startX}, {selection.startY}</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-slate-400">End</span>
                        <span className="text-white font-mono">{selection.endX}, {selection.endY}</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-slate-400">Size</span>
                        <span className="text-white font-mono">
                          {Math.abs(selection.endX - selection.startX) + 1} x {Math.abs(selection.endY - selection.startY) + 1}
                        </span>
                      </div>
                    </div>
                  </div>
                ) : hoveredTile && map?.tiles.get(`${hoveredTile.x}:${hoveredTile.y}:${hoveredTile.z}`) ? (
                  <div className="space-y-3">
                    <h4 className="text-xs font-medium text-slate-500 uppercase">Tile Properties</h4>
                    <div className="bg-slate-800/50 rounded-lg p-3 space-y-2 text-sm">
                      <div className="flex justify-between">
                        <span className="text-slate-400">Position</span>
                        <span className="text-white font-mono">{hoveredTile.x}, {hoveredTile.y}, {hoveredTile.z}</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-slate-400">Tile ID</span>
                        <span className="text-white font-mono">
                          {map.tiles.get(`${hoveredTile.x}:${hoveredTile.y}:${hoveredTile.z}`)?.tileId || 'None'}
                        </span>
                      </div>
                    </div>
                  </div>
                ) : (
                  <div className="text-center text-slate-500 text-sm py-8">
                    <p>Hover over a tile or make a selection to view properties</p>
                </div>
                )}
              </Tabs.Content>
            </Tabs.Root>

            {/* Bottom Actions */}
            <div className="p-3 border-t border-slate-700">
              <div className="grid grid-cols-2 gap-2">
                <button 
                  onClick={() => copySelection()}
                  disabled={!selection}
                  className="flex items-center justify-center gap-1 px-3 py-2 bg-slate-800 text-slate-300 rounded-lg hover:bg-slate-700 transition text-xs disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <Copy className="w-3 h-3" />
                  <span>Copy</span>
                </button>
                <button 
                  onClick={() => setSelection(null)}
                  disabled={!selection}
                  className="flex items-center justify-center gap-1 px-3 py-2 bg-slate-800 text-slate-300 rounded-lg hover:bg-slate-700 transition text-xs disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <Trash2 className="w-3 h-3" />
                  <span>Clear</span>
                </button>
              </div>
            </div>
          </aside>
        </div>

        {/* Status Bar */}
        <footer className="h-6 bg-slate-900/90 border-t border-slate-700 flex items-center justify-between px-4 text-xs text-slate-400">
          <div className="flex items-center space-x-4">
            <span>Tool: {tools.find(t => t.id === activeTool)?.name}</span>
            <span className="text-slate-600">|</span>
            <span>Brush: #{brush.tileId}</span>
            {selection && (
              <>
                <span className="text-slate-600">|</span>
                <span>Selection: {Math.abs(selection.endX - selection.startX) + 1}x{Math.abs(selection.endY - selection.startY) + 1}</span>
              </>
            )}
          </div>
          <div className="flex items-center space-x-4">
            <span className="flex items-center gap-1">
              {isDirty && <span className="w-2 h-2 bg-amber-500 rounded-full" />}
              Map: {fileName || 'Untitled.otbm'}
            </span>
            <span className="text-slate-600">|</span>
            <span>Size: {map?.header.width || 0}x{map?.header.height || 0}</span>
            <span className="text-slate-600">|</span>
            <span>Tiles: {map?.tiles.size.toLocaleString() || 0}</span>
          </div>
        </footer>
      </div>
    </Tooltip.Provider>
  )
}
