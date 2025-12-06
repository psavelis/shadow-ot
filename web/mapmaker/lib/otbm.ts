// OTBM (Open Tibia Binary Map) Parser and Serializer

export interface OTBMHeader {
  version: number
  width: number
  height: number
  itemsMajorVersion: number
  itemsMinorVersion: number
}

export interface OTBMTile {
  x: number
  y: number
  z: number
  tileId?: number
  flags?: number
  items: OTBMItem[]
}

export interface OTBMItem {
  id: number
  count?: number
  actionId?: number
  uniqueId?: number
  text?: string
  attributes: Map<number, any>
}

export interface OTBMTown {
  id: number
  name: string
  templePosition: { x: number; y: number; z: number }
}

export interface OTBMWaypoint {
  name: string
  position: { x: number; y: number; z: number }
}

export interface OTBMMap {
  header: OTBMHeader
  description: string
  spawnsFile: string
  housesFile: string
  tiles: Map<string, OTBMTile>
  towns: OTBMTown[]
  waypoints: OTBMWaypoint[]
}

// OTBM Node Types
const OTBM_NODE = {
  ROOT: 0x00,
  MAP_DATA: 0x02,
  TILE_AREA: 0x04,
  TILE: 0x05,
  ITEM: 0x06,
  TOWNS: 0x0C,
  TOWN: 0x0D,
  HOUSE_TILE: 0x0E,
  WAYPOINTS: 0x0F,
  WAYPOINT: 0x10,
} as const

// OTBM Attributes
const OTBM_ATTR = {
  DESCRIPTION: 0x01,
  EXT_FILE: 0x02,
  TILE_FLAGS: 0x03,
  ACTION_ID: 0x04,
  UNIQUE_ID: 0x05,
  TEXT: 0x06,
  DESC: 0x07,
  DEST: 0x08,
  ITEM: 0x09,
  DEPOT_ID: 0x0A,
  SPAWN_FILE: 0x0B,
  RUNE_CHARGES: 0x0C,
  HOUSE_FILE: 0x0D,
  HOUSE_DOOR_ID: 0x0E,
  COUNT: 0x0F,
  DURATION: 0x10,
  DECAYING_STATE: 0x11,
  WRITTEN_DATE: 0x12,
  WRITTEN_BY: 0x13,
  SLEEPER_GUID: 0x14,
  SLEEP_START: 0x15,
  CHARGES: 0x16,
} as const

// Tile flags
export const TILE_FLAGS = {
  PROTECTION_ZONE: 1 << 0,
  NO_PVP_ZONE: 1 << 2,
  NO_LOGOUT: 1 << 3,
  PVP_ZONE: 1 << 4,
  REFRESH: 1 << 5,
} as const

class BinaryReader {
  private buffer: Uint8Array
  private position: number = 0

  constructor(buffer: ArrayBuffer) {
    this.buffer = new Uint8Array(buffer)
  }

  get remaining(): number {
    return this.buffer.length - this.position
  }

  readUint8(): number {
    return this.buffer[this.position++]
  }

  readUint16(): number {
    const value = this.buffer[this.position] | (this.buffer[this.position + 1] << 8)
    this.position += 2
    return value
  }

  readUint32(): number {
    const value = 
      this.buffer[this.position] |
      (this.buffer[this.position + 1] << 8) |
      (this.buffer[this.position + 2] << 16) |
      (this.buffer[this.position + 3] << 24)
    this.position += 4
    return value >>> 0
  }

  readString(): string {
    const length = this.readUint16()
    const bytes = this.buffer.slice(this.position, this.position + length)
    this.position += length
    return new TextDecoder().decode(bytes)
  }

  readBytes(length: number): Uint8Array {
    const bytes = this.buffer.slice(this.position, this.position + length)
    this.position += length
    return bytes
  }

  skip(bytes: number): void {
    this.position += bytes
  }

  peek(): number {
    return this.buffer[this.position]
  }
}

class BinaryWriter {
  private chunks: Uint8Array[] = []
  private currentChunk: number[] = []

  writeUint8(value: number): void {
    this.currentChunk.push(value & 0xFF)
  }

  writeUint16(value: number): void {
    this.currentChunk.push(value & 0xFF)
    this.currentChunk.push((value >> 8) & 0xFF)
  }

  writeUint32(value: number): void {
    this.currentChunk.push(value & 0xFF)
    this.currentChunk.push((value >> 8) & 0xFF)
    this.currentChunk.push((value >> 16) & 0xFF)
    this.currentChunk.push((value >> 24) & 0xFF)
  }

  writeString(str: string): void {
    const bytes = new TextEncoder().encode(str)
    this.writeUint16(bytes.length)
    for (const byte of bytes) {
      this.currentChunk.push(byte)
    }
  }

  writeBytes(bytes: Uint8Array): void {
    for (const byte of bytes) {
      this.currentChunk.push(byte)
    }
  }

  toBuffer(): ArrayBuffer {
    const totalLength = this.currentChunk.length
    const buffer = new ArrayBuffer(totalLength)
    const view = new Uint8Array(buffer)
    view.set(this.currentChunk)
    return buffer
  }
}

export function getTileKey(x: number, y: number, z: number): string {
  return `${x}:${y}:${z}`
}

export function parseTileKey(key: string): { x: number; y: number; z: number } {
  const [x, y, z] = key.split(':').map(Number)
  return { x, y, z }
}

export async function parseOTBM(buffer: ArrayBuffer): Promise<OTBMMap> {
  const reader = new BinaryReader(buffer)
  
  // Read and validate file identifier
  const identifier = reader.readUint32()
  if (identifier !== 0x00000000) {
    // Check for OTBM magic number or bare format
  }

  const map: OTBMMap = {
    header: {
      version: 0,
      width: 0,
      height: 0,
      itemsMajorVersion: 0,
      itemsMinorVersion: 0,
    },
    description: '',
    spawnsFile: '',
    housesFile: '',
    tiles: new Map(),
    towns: [],
    waypoints: [],
  }

  // Parse nodes recursively
  const parseNode = (parentType: number) => {
    while (reader.remaining > 0) {
      const marker = reader.readUint8()
      
      if (marker === 0xFE) {
        // Start node
        const nodeType = reader.readUint8()
        
        switch (nodeType) {
          case OTBM_NODE.ROOT:
            map.header.version = reader.readUint32()
            map.header.width = reader.readUint16()
            map.header.height = reader.readUint16()
            map.header.itemsMajorVersion = reader.readUint32()
            map.header.itemsMinorVersion = reader.readUint32()
            parseNode(OTBM_NODE.ROOT)
            break
            
          case OTBM_NODE.MAP_DATA:
            parseMapData()
            parseNode(OTBM_NODE.MAP_DATA)
            break
            
          case OTBM_NODE.TILE_AREA:
            const areaX = reader.readUint16()
            const areaY = reader.readUint16()
            const areaZ = reader.readUint8()
            parseTileArea(areaX, areaY, areaZ)
            break
            
          case OTBM_NODE.TOWNS:
            parseNode(OTBM_NODE.TOWNS)
            break
            
          case OTBM_NODE.TOWN:
            parseTown()
            break
            
          case OTBM_NODE.WAYPOINTS:
            parseNode(OTBM_NODE.WAYPOINTS)
            break
            
          case OTBM_NODE.WAYPOINT:
            parseWaypoint()
            break
        }
      } else if (marker === 0xFF) {
        // End node
        return
      }
    }
  }

  const parseMapData = () => {
    while (reader.remaining > 0 && reader.peek() !== 0xFE && reader.peek() !== 0xFF) {
      const attr = reader.readUint8()
      
      switch (attr) {
        case OTBM_ATTR.DESCRIPTION:
          map.description = reader.readString()
          break
        case OTBM_ATTR.SPAWN_FILE:
          map.spawnsFile = reader.readString()
          break
        case OTBM_ATTR.HOUSE_FILE:
          map.housesFile = reader.readString()
          break
        default:
          // Skip unknown attribute
          break
      }
    }
  }

  const parseTileArea = (baseX: number, baseY: number, z: number) => {
    while (reader.remaining > 0) {
      const marker = reader.readUint8()
      
      if (marker === 0xFE) {
        const nodeType = reader.readUint8()
        
        if (nodeType === OTBM_NODE.TILE || nodeType === OTBM_NODE.HOUSE_TILE) {
          const offsetX = reader.readUint8()
          const offsetY = reader.readUint8()
          
          const tile: OTBMTile = {
            x: baseX + offsetX,
            y: baseY + offsetY,
            z: z,
            items: [],
          }
          
          // Parse tile attributes
          while (reader.remaining > 0 && reader.peek() !== 0xFE && reader.peek() !== 0xFF) {
            const attr = reader.readUint8()
            
            switch (attr) {
              case OTBM_ATTR.TILE_FLAGS:
                tile.flags = reader.readUint32()
                break
              case OTBM_ATTR.ITEM:
                tile.tileId = reader.readUint16()
                break
              default:
                break
            }
          }
          
          // Parse tile items
          parseTileItems(tile)
          
          map.tiles.set(getTileKey(tile.x, tile.y, tile.z), tile)
        }
      } else if (marker === 0xFF) {
        return
      }
    }
  }

  const parseTileItems = (tile: OTBMTile) => {
    while (reader.remaining > 0) {
      const marker = reader.readUint8()
      
      if (marker === 0xFE) {
        const nodeType = reader.readUint8()
        
        if (nodeType === OTBM_NODE.ITEM) {
          const item: OTBMItem = {
            id: reader.readUint16(),
            attributes: new Map(),
          }
          
          // Parse item attributes
          while (reader.remaining > 0 && reader.peek() !== 0xFE && reader.peek() !== 0xFF) {
            const attr = reader.readUint8()
            
            switch (attr) {
              case OTBM_ATTR.COUNT:
                item.count = reader.readUint8()
                break
              case OTBM_ATTR.ACTION_ID:
                item.actionId = reader.readUint16()
                break
              case OTBM_ATTR.UNIQUE_ID:
                item.uniqueId = reader.readUint16()
                break
              case OTBM_ATTR.TEXT:
              case OTBM_ATTR.DESC:
                item.text = reader.readString()
                break
              default:
                item.attributes.set(attr, true)
                break
            }
          }
          
          tile.items.push(item)
          
          // Parse nested items (containers)
          parseTileItems(tile)
        }
      } else if (marker === 0xFF) {
        return
      }
    }
  }

  const parseTown = () => {
    const town: OTBMTown = {
      id: reader.readUint32(),
      name: reader.readString(),
      templePosition: {
        x: reader.readUint16(),
        y: reader.readUint16(),
        z: reader.readUint8(),
      },
    }
    map.towns.push(town)
  }

  const parseWaypoint = () => {
    const waypoint: OTBMWaypoint = {
      name: reader.readString(),
      position: {
        x: reader.readUint16(),
        y: reader.readUint16(),
        z: reader.readUint8(),
      },
    }
    map.waypoints.push(waypoint)
  }

  try {
    parseNode(-1)
  } catch (e) {
    console.error('Error parsing OTBM:', e)
  }

  return map
}

export function serializeOTBM(map: OTBMMap): ArrayBuffer {
  const writer = new BinaryWriter()

  // Write file identifier
  writer.writeUint32(0x00000000)

  // Write start node marker
  writer.writeUint8(0xFE)
  
  // Write root node
  writer.writeUint8(OTBM_NODE.ROOT)
  writer.writeUint32(map.header.version)
  writer.writeUint16(map.header.width)
  writer.writeUint16(map.header.height)
  writer.writeUint32(map.header.itemsMajorVersion)
  writer.writeUint32(map.header.itemsMinorVersion)

  // Write map data node
  writer.writeUint8(0xFE)
  writer.writeUint8(OTBM_NODE.MAP_DATA)

  if (map.description) {
    writer.writeUint8(OTBM_ATTR.DESCRIPTION)
    writer.writeString(map.description)
  }

  if (map.spawnsFile) {
    writer.writeUint8(OTBM_ATTR.SPAWN_FILE)
    writer.writeString(map.spawnsFile)
  }

  if (map.housesFile) {
    writer.writeUint8(OTBM_ATTR.HOUSE_FILE)
    writer.writeString(map.housesFile)
  }

  // Group tiles by area (256x256)
  const areas = new Map<string, OTBMTile[]>()
  
  for (const [key, tile] of map.tiles) {
    const areaX = Math.floor(tile.x / 256) * 256
    const areaY = Math.floor(tile.y / 256) * 256
    const areaKey = `${areaX}:${areaY}:${tile.z}`
    
    if (!areas.has(areaKey)) {
      areas.set(areaKey, [])
    }
    areas.get(areaKey)!.push(tile)
  }

  // Write tile areas
  for (const [areaKey, tiles] of areas) {
    const [areaX, areaY, z] = areaKey.split(':').map(Number)
    
    writer.writeUint8(0xFE)
    writer.writeUint8(OTBM_NODE.TILE_AREA)
    writer.writeUint16(areaX)
    writer.writeUint16(areaY)
    writer.writeUint8(z)

    for (const tile of tiles) {
      writer.writeUint8(0xFE)
      writer.writeUint8(OTBM_NODE.TILE)
      writer.writeUint8(tile.x - areaX)
      writer.writeUint8(tile.y - areaY)

      if (tile.flags) {
        writer.writeUint8(OTBM_ATTR.TILE_FLAGS)
        writer.writeUint32(tile.flags)
      }

      if (tile.tileId) {
        writer.writeUint8(OTBM_ATTR.ITEM)
        writer.writeUint16(tile.tileId)
      }

      // Write items
      for (const item of tile.items) {
        writer.writeUint8(0xFE)
        writer.writeUint8(OTBM_NODE.ITEM)
        writer.writeUint16(item.id)

        if (item.count) {
          writer.writeUint8(OTBM_ATTR.COUNT)
          writer.writeUint8(item.count)
        }

        if (item.actionId) {
          writer.writeUint8(OTBM_ATTR.ACTION_ID)
          writer.writeUint16(item.actionId)
        }

        if (item.uniqueId) {
          writer.writeUint8(OTBM_ATTR.UNIQUE_ID)
          writer.writeUint16(item.uniqueId)
        }

        if (item.text) {
          writer.writeUint8(OTBM_ATTR.TEXT)
          writer.writeString(item.text)
        }

        writer.writeUint8(0xFF) // End item
      }

      writer.writeUint8(0xFF) // End tile
    }

    writer.writeUint8(0xFF) // End tile area
  }

  // Write towns
  if (map.towns.length > 0) {
    writer.writeUint8(0xFE)
    writer.writeUint8(OTBM_NODE.TOWNS)

    for (const town of map.towns) {
      writer.writeUint8(0xFE)
      writer.writeUint8(OTBM_NODE.TOWN)
      writer.writeUint32(town.id)
      writer.writeString(town.name)
      writer.writeUint16(town.templePosition.x)
      writer.writeUint16(town.templePosition.y)
      writer.writeUint8(town.templePosition.z)
      writer.writeUint8(0xFF) // End town
    }

    writer.writeUint8(0xFF) // End towns
  }

  // Write waypoints
  if (map.waypoints.length > 0) {
    writer.writeUint8(0xFE)
    writer.writeUint8(OTBM_NODE.WAYPOINTS)

    for (const waypoint of map.waypoints) {
      writer.writeUint8(0xFE)
      writer.writeUint8(OTBM_NODE.WAYPOINT)
      writer.writeString(waypoint.name)
      writer.writeUint16(waypoint.position.x)
      writer.writeUint16(waypoint.position.y)
      writer.writeUint8(waypoint.position.z)
      writer.writeUint8(0xFF) // End waypoint
    }

    writer.writeUint8(0xFF) // End waypoints
  }

  writer.writeUint8(0xFF) // End map data
  writer.writeUint8(0xFF) // End root

  return writer.toBuffer()
}

export function createEmptyMap(width: number, height: number): OTBMMap {
  return {
    header: {
      version: 2,
      width,
      height,
      itemsMajorVersion: 3,
      itemsMinorVersion: 57,
    },
    description: 'Created with Shadow OT Map Maker',
    spawnsFile: 'map-spawn.xml',
    housesFile: 'map-house.xml',
    tiles: new Map(),
    towns: [],
    waypoints: [],
  }
}

