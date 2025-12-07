'use client'

import { useRef, useEffect, useCallback, useState } from 'react'
import { useMapStore, Tool } from '@/lib/mapStore'
import { getTileKey } from '@/lib/otbm'

// Tile size in pixels
const TILE_SIZE = 32

// Simple tile colors for demonstration (in production, use actual sprite sheets)
const TILE_COLORS: Record<number, string> = {
  4526: '#2d5a27', // Grass
  4527: '#1a4d1a', // Dark grass
  4405: '#8b7355', // Dirt
  4406: '#6b5344', // Dark dirt
  4820: '#3366cc', // Water
  4821: '#2255bb', // Deep water
  4835: '#cccccc', // Stone
  4836: '#999999', // Dark stone
  4907: '#e6cc80', // Sand
  4908: '#d4b366', // Dark sand
  105: '#111111', // Void
}

export function MapCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const containerRef = useRef<HTMLDivElement>(null)
  const [isPanning, setIsPanning] = useState(false)
  const [isDrawing, setIsDrawing] = useState(false)
  const [lastMousePos, setLastMousePos] = useState({ x: 0, y: 0 })
  const [selectionStart, setSelectionStart] = useState<{ x: number; y: number } | null>(null)

  const {
    map,
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
    showZones,
    showGrid,
    setViewport,
    setZoom,
    setSelection,
    setHoveredTile,
    placeTile,
    eraseTile,
    fillArea,
  } = useMapStore()

  // Get tile position from mouse coordinates
  const getTileFromMouse = useCallback((clientX: number, clientY: number) => {
    const canvas = canvasRef.current
    if (!canvas) return null

    const rect = canvas.getBoundingClientRect()
    const x = (clientX - rect.left) / zoom
    const y = (clientY - rect.top) / zoom

    const tileX = Math.floor(x / TILE_SIZE) + viewportX - Math.floor(canvas.width / zoom / TILE_SIZE / 2)
    const tileY = Math.floor(y / TILE_SIZE) + viewportY - Math.floor(canvas.height / zoom / TILE_SIZE / 2)

    return { x: tileX, y: tileY, z: currentFloor }
  }, [zoom, viewportX, viewportY, currentFloor])

  // Render the map
  const render = useCallback(() => {
    const canvas = canvasRef.current
    const ctx = canvas?.getContext('2d')
    if (!canvas || !ctx) return

    // Clear canvas
    ctx.fillStyle = '#1a1a2e'
    ctx.fillRect(0, 0, canvas.width, canvas.height)

    if (!map) {
      // Draw "No map loaded" message
      ctx.fillStyle = '#666'
      ctx.font = '24px sans-serif'
      ctx.textAlign = 'center'
      ctx.fillText('No map loaded', canvas.width / 2, canvas.height / 2)
      ctx.font = '14px sans-serif'
      ctx.fillText('Create a new map or open an existing OTBM file', canvas.width / 2, canvas.height / 2 + 30)
      return
    }

    const scaledTileSize = TILE_SIZE * zoom
    const tilesX = Math.ceil(canvas.width / scaledTileSize) + 1
    const tilesY = Math.ceil(canvas.height / scaledTileSize) + 1

    const startX = viewportX - Math.floor(tilesX / 2)
    const startY = viewportY - Math.floor(tilesY / 2)

    ctx.save()
    ctx.scale(zoom, zoom)

    // Draw tiles
    for (let dx = 0; dx < tilesX; dx++) {
      for (let dy = 0; dy < tilesY; dy++) {
        const tileX = startX + dx
        const tileY = startY + dy
        const key = getTileKey(tileX, tileY, currentFloor)
        const tile = map.tiles.get(key)

        const screenX = dx * TILE_SIZE
        const screenY = dy * TILE_SIZE

        if (tile && showGround && tile.tileId) {
          const color = TILE_COLORS[tile.tileId] || '#333'
          ctx.fillStyle = color
          ctx.fillRect(screenX, screenY, TILE_SIZE, TILE_SIZE)

          // Draw zone overlay
          if (showZones && tile.flags) {
            ctx.fillStyle = 'rgba(255, 0, 0, 0.2)'
            if (tile.flags & 1) { // Protection zone
              ctx.fillStyle = 'rgba(0, 255, 0, 0.2)'
            }
            ctx.fillRect(screenX, screenY, TILE_SIZE, TILE_SIZE)
          }

          // Draw items (simplified)
          if (showItems && tile.items.length > 0) {
            ctx.fillStyle = 'rgba(255, 255, 0, 0.3)'
            ctx.beginPath()
            ctx.arc(screenX + TILE_SIZE / 2, screenY + TILE_SIZE / 2, 8, 0, Math.PI * 2)
            ctx.fill()
          }
        }

        // Draw grid
        if (showGrid) {
          ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)'
          ctx.lineWidth = 1 / zoom
          ctx.strokeRect(screenX, screenY, TILE_SIZE, TILE_SIZE)
        }
      }
    }

    // Draw selection
    if (selection && selection.z === currentFloor) {
      const selMinX = Math.min(selection.startX, selection.endX)
      const selMaxX = Math.max(selection.startX, selection.endX)
      const selMinY = Math.min(selection.startY, selection.endY)
      const selMaxY = Math.max(selection.startY, selection.endY)

      const screenStartX = (selMinX - startX) * TILE_SIZE
      const screenStartY = (selMinY - startY) * TILE_SIZE
      const width = (selMaxX - selMinX + 1) * TILE_SIZE
      const height = (selMaxY - selMinY + 1) * TILE_SIZE

      ctx.fillStyle = 'rgba(59, 130, 246, 0.2)'
      ctx.fillRect(screenStartX, screenStartY, width, height)

      ctx.strokeStyle = 'rgba(59, 130, 246, 0.8)'
      ctx.lineWidth = 2 / zoom
      ctx.setLineDash([5 / zoom, 5 / zoom])
      ctx.strokeRect(screenStartX, screenStartY, width, height)
      ctx.setLineDash([])
    }

    // Draw hover highlight
    if (hoveredTile && hoveredTile.z === currentFloor) {
      const screenX = (hoveredTile.x - startX) * TILE_SIZE
      const screenY = (hoveredTile.y - startY) * TILE_SIZE

      ctx.strokeStyle = activeTool === 'erase' 
        ? 'rgba(239, 68, 68, 0.8)' 
        : 'rgba(251, 191, 36, 0.8)'
      ctx.lineWidth = 2 / zoom
      ctx.strokeRect(screenX, screenY, TILE_SIZE * brush.size, TILE_SIZE * brush.size)

      // Draw brush preview
      if (activeTool === 'brush') {
        const color = TILE_COLORS[brush.tileId] || '#666'
        ctx.fillStyle = color + '80' // 50% opacity
        ctx.fillRect(screenX, screenY, TILE_SIZE * brush.size, TILE_SIZE * brush.size)
      }
    }

    ctx.restore()

    // Draw coordinates overlay
    if (hoveredTile) {
      ctx.fillStyle = 'rgba(0, 0, 0, 0.7)'
      ctx.fillRect(10, canvas.height - 30, 200, 25)
      ctx.fillStyle = '#fff'
      ctx.font = '12px monospace'
      ctx.fillText(`X: ${hoveredTile.x}  Y: ${hoveredTile.y}  Z: ${currentFloor}`, 20, canvas.height - 12)
    }
  }, [map, viewportX, viewportY, zoom, currentFloor, showGround, showItems, showZones, showGrid, selection, hoveredTile, activeTool, brush])

  // Animation frame for smooth rendering
  useEffect(() => {
    let animationId: number

    const animate = () => {
      render()
      animationId = requestAnimationFrame(animate)
    }

    animate()

    return () => cancelAnimationFrame(animationId)
  }, [render])

  // Handle resize
  useEffect(() => {
    const handleResize = () => {
      const canvas = canvasRef.current
      const container = containerRef.current
      if (!canvas || !container) return

      canvas.width = container.clientWidth
      canvas.height = container.clientHeight
    }

    handleResize()
    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [])

  // Mouse handlers
  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    const tile = getTileFromMouse(e.clientX, e.clientY)
    
    if (e.button === 1 || (e.button === 0 && e.altKey)) {
      // Middle click or Alt+Left click = pan
      setIsPanning(true)
      setLastMousePos({ x: e.clientX, y: e.clientY })
    } else if (e.button === 0 && tile) {
      if (activeTool === 'select') {
        setSelectionStart({ x: tile.x, y: tile.y })
        setSelection({
          startX: tile.x,
          startY: tile.y,
          endX: tile.x,
          endY: tile.y,
          z: currentFloor,
        })
      } else if (activeTool === 'brush' || activeTool === 'erase') {
        setIsDrawing(true)
        if (activeTool === 'brush') {
          placeTile(tile.x, tile.y, tile.z)
        } else {
          eraseTile(tile.x, tile.y, tile.z)
        }
      } else if (activeTool === 'fill') {
        // Start fill selection
        setSelectionStart({ x: tile.x, y: tile.y })
      }
    }
  }, [getTileFromMouse, activeTool, currentFloor, placeTile, eraseTile, setSelection])

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    const tile = getTileFromMouse(e.clientX, e.clientY)
    
    if (tile) {
      setHoveredTile(tile)
    }

    if (isPanning) {
      const dx = (e.clientX - lastMousePos.x) / zoom / TILE_SIZE
      const dy = (e.clientY - lastMousePos.y) / zoom / TILE_SIZE
      setViewport(viewportX - dx, viewportY - dy)
      setLastMousePos({ x: e.clientX, y: e.clientY })
    } else if (isDrawing && tile) {
      if (activeTool === 'brush') {
        placeTile(tile.x, tile.y, tile.z)
      } else if (activeTool === 'erase') {
        eraseTile(tile.x, tile.y, tile.z)
      }
    } else if (selectionStart && tile) {
      setSelection({
        startX: selectionStart.x,
        startY: selectionStart.y,
        endX: tile.x,
        endY: tile.y,
        z: currentFloor,
      })
    }
  }, [getTileFromMouse, isPanning, isDrawing, selectionStart, zoom, lastMousePos, viewportX, viewportY, activeTool, currentFloor, setViewport, setHoveredTile, setSelection, placeTile, eraseTile])

  const handleMouseUp = useCallback((e: React.MouseEvent) => {
    if (isPanning) {
      setIsPanning(false)
    }
    
    if (isDrawing) {
      setIsDrawing(false)
    }
    
    if (selectionStart && activeTool === 'fill' && selection) {
      fillArea(selection.startX, selection.startY, selection.endX, selection.endY, currentFloor)
      setSelection(null)
    }
    
    setSelectionStart(null)
  }, [isPanning, isDrawing, selectionStart, activeTool, selection, currentFloor, fillArea, setSelection])

  const handleMouseLeave = useCallback(() => {
    setHoveredTile(null)
    setIsDrawing(false)
    setIsPanning(false)
  }, [setHoveredTile])

  const handleWheel = useCallback((e: React.WheelEvent) => {
    e.preventDefault()
    
    if (e.ctrlKey) {
      // Zoom
      const delta = e.deltaY > 0 ? -0.1 : 0.1
      setZoom(zoom + delta)
    } else {
      // Pan
      const dx = e.deltaX / zoom / TILE_SIZE
      const dy = e.deltaY / zoom / TILE_SIZE
      setViewport(viewportX + dx, viewportY + dy)
    }
  }, [zoom, viewportX, viewportY, setZoom, setViewport])

  return (
    <div 
      ref={containerRef}
      className="w-full h-full relative overflow-hidden bg-slate-950"
    >
      <canvas
        ref={canvasRef}
        className="absolute inset-0 cursor-crosshair"
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseLeave}
        onWheel={handleWheel}
        onContextMenu={e => e.preventDefault()}
      />
    </div>
  )
}


