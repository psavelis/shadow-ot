/**
 * Asset utilities for Shadow OT
 * Uses open-source Tibia assets from various sources:
 * - TibiaData API (https://tibiadata.com)
 * - TibiaMaps (https://tibiamaps.github.io/tibia-map-data/)
 * - OpenTibia sprites (https://github.com/opentibiabr/canary)
 */

// TibiaData API base URL
const TIBIA_DATA_API = 'https://api.tibiadata.com/v3'

// Open-source sprite repositories
const SPRITE_SOURCES = {
  // OTClient sprites repository
  otclient: 'https://raw.githubusercontent.com/mehah/otclient/main/data/things',
  // TibiaMaps creature images
  tibiamaps: 'https://tibiamaps.github.io/tibia-map-data/creature-images',
  // Tibia Wiki images (fallback)
  tibiawiki: 'https://static.tibia.com/images',
}

/**
 * Get creature sprite URL from open-source repositories
 */
export function getCreatureSprite(creatureName: string, size: 'small' | 'medium' | 'large' = 'medium'): string {
  const normalizedName = creatureName.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '')
  
  // TibiaMaps has high-quality creature images
  return `${SPRITE_SOURCES.tibiamaps}/${normalizedName}.png`
}

/**
 * Get item sprite URL
 */
export function getItemSprite(itemId: number): string {
  // Using OTClient data format
  return `${SPRITE_SOURCES.otclient}/items/${itemId}.png`
}

/**
 * Get spell icon based on element
 */
export function getSpellIcon(element: string): string {
  const elementIcons: Record<string, string> = {
    fire: '/icons/elements/fire.svg',
    ice: '/icons/elements/ice.svg',
    energy: '/icons/elements/energy.svg',
    earth: '/icons/elements/earth.svg',
    holy: '/icons/elements/holy.svg',
    death: '/icons/elements/death.svg',
    physical: '/icons/elements/physical.svg',
    healing: '/icons/elements/healing.svg',
  }
  return elementIcons[element] || elementIcons.physical
}

/**
 * Get outfit sprite for a character
 */
export function getOutfitSprite(outfit: {
  lookType: number
  lookHead: number
  lookBody: number
  lookLegs: number
  lookFeet: number
  lookAddons?: number
  lookMount?: number
}): string {
  // Using TibiaData outfit images API
  const params = new URLSearchParams({
    type: outfit.lookType.toString(),
    head: outfit.lookHead.toString(),
    body: outfit.lookBody.toString(),
    legs: outfit.lookLegs.toString(),
    feet: outfit.lookFeet.toString(),
    addons: (outfit.lookAddons || 0).toString(),
  })
  
  if (outfit.lookMount) {
    params.append('mount', outfit.lookMount.toString())
  }
  
  return `https://outfit-images.ots.me/1.4/animatedoutfit.php?${params.toString()}`
}

/**
 * Get vocation icon
 */
export function getVocationIcon(vocation: string): string {
  const normalizedVocation = vocation.toLowerCase().replace(/\s+/g, '-')
  
  const vocationMap: Record<string, string> = {
    'none': '/icons/vocations/none.svg',
    'knight': '/icons/vocations/knight.svg',
    'elite-knight': '/icons/vocations/knight.svg',
    'paladin': '/icons/vocations/paladin.svg',
    'royal-paladin': '/icons/vocations/paladin.svg',
    'sorcerer': '/icons/vocations/sorcerer.svg',
    'master-sorcerer': '/icons/vocations/sorcerer.svg',
    'druid': '/icons/vocations/druid.svg',
    'elder-druid': '/icons/vocations/druid.svg',
  }
  
  return vocationMap[normalizedVocation] || vocationMap.none
}

/**
 * Get map tile image
 */
export function getMapTile(x: number, y: number, z: number): string {
  // TibiaMaps provides map tiles
  return `https://tibiamaps.github.io/tibia-map-data/map/${z}/${x}-${y}.png`
}

/**
 * Fetch creature data from TibiaData API
 */
export async function fetchCreatureData(creatureName: string) {
  const response = await fetch(`${TIBIA_DATA_API}/creature/${encodeURIComponent(creatureName)}`)
  if (!response.ok) throw new Error('Creature not found')
  return response.json()
}

/**
 * Fetch spell data from TibiaData API
 */
export async function fetchSpellData(spellName: string) {
  const response = await fetch(`${TIBIA_DATA_API}/spell/${encodeURIComponent(spellName)}`)
  if (!response.ok) throw new Error('Spell not found')
  return response.json()
}

/**
 * Fetch all spells from TibiaData API
 */
export async function fetchAllSpells() {
  const response = await fetch(`${TIBIA_DATA_API}/spells`)
  if (!response.ok) throw new Error('Failed to fetch spells')
  return response.json()
}

/**
 * Fetch all creatures from TibiaData API
 */
export async function fetchAllCreatures() {
  const response = await fetch(`${TIBIA_DATA_API}/creatures`)
  if (!response.ok) throw new Error('Failed to fetch creatures')
  return response.json()
}

/**
 * Fetch boosted creature from TibiaData API
 */
export async function fetchBoostedCreature() {
  const response = await fetch(`${TIBIA_DATA_API}/boostablebosses`)
  if (!response.ok) throw new Error('Failed to fetch boosted creature')
  return response.json()
}

/**
 * Get realm-specific theme colors
 */
export function getRealmColors(realmId: string): { primary: string; secondary: string; gradient: string } {
  const realmThemes: Record<string, { primary: string; secondary: string; gradient: string }> = {
    shadowveil: { primary: '#8b5cf6', secondary: '#6366f1', gradient: 'from-purple-500 to-indigo-600' },
    aetheria: { primary: '#06b6d4', secondary: '#0ea5e9', gradient: 'from-cyan-500 to-blue-500' },
    warbound: { primary: '#ef4444', secondary: '#f97316', gradient: 'from-red-500 to-orange-500' },
    mythara: { primary: '#10b981', secondary: '#14b8a6', gradient: 'from-emerald-500 to-teal-500' },
    voidborne: { primary: '#6366f1', secondary: '#8b5cf6', gradient: 'from-indigo-500 to-purple-600' },
    grimhollow: { primary: '#64748b', secondary: '#475569', gradient: 'from-slate-500 to-slate-700' },
  }
  
  return realmThemes[realmId] || realmThemes.shadowveil
}

/**
 * Format time remaining until reset
 */
export function getTimeUntilReset(): string {
  const now = new Date()
  const reset = new Date()
  reset.setUTCHours(10, 0, 0, 0) // Server save is at 10:00 UTC
  
  if (now >= reset) {
    reset.setDate(reset.getDate() + 1)
  }
  
  const diff = reset.getTime() - now.getTime()
  const hours = Math.floor(diff / (1000 * 60 * 60))
  const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60))
  
  return `${hours}h ${minutes}m`
}

