// ============================================
// Core Types for Shadow OT Web Applications
// ============================================

// User & Authentication
export interface User {
  id: string
  email: string
  username: string
  premiumUntil: string | null
  coins: number
  createdAt: string
  lastLogin: string
  twoFactorEnabled: boolean
  walletAddress?: string
}

export interface AuthTokens {
  accessToken: string
  refreshToken: string
  expiresAt: number
}

export interface LoginRequest {
  email: string
  password: string
  twoFactorCode?: string
}

export interface RegisterRequest {
  email: string
  username: string
  password: string
  confirmPassword: string
}

// Characters
export interface Character {
  id: string
  name: string
  level: number
  vocation: Vocation
  sex: 'male' | 'female'
  realm: RealmId
  online: boolean
  lastLogin: string
  createdAt: string
  experience: number
  health: number
  healthMax: number
  mana: number
  manaMax: number
  capacity: number
  skills: CharacterSkills
  outfit: Outfit
  guild?: GuildMembership
  achievements: number
  deaths: number
  kills: number
}

export interface CharacterSkills {
  fist: SkillValue
  club: SkillValue
  sword: SkillValue
  axe: SkillValue
  distance: SkillValue
  shielding: SkillValue
  fishing: SkillValue
  magicLevel: SkillValue
}

export interface SkillValue {
  level: number
  percent: number
}

export interface Outfit {
  lookType: number
  lookHead: number
  lookBody: number
  lookLegs: number
  lookFeet: number
  lookAddons: number
  lookMount?: number
}

export type Vocation = 
  | 'None'
  | 'Knight' 
  | 'Elite Knight'
  | 'Paladin'
  | 'Royal Paladin'
  | 'Sorcerer'
  | 'Master Sorcerer'
  | 'Druid'
  | 'Elder Druid'

// Realms
export type RealmId = 
  | 'shadowveil'
  | 'aetheria'
  | 'warbound'
  | 'mythara'
  | 'voidborne'
  | 'grimhollow'

export interface Realm {
  id: RealmId
  name: string
  description: string
  theme: RealmTheme
  status: 'online' | 'offline' | 'maintenance'
  playersOnline: number
  playersRecord: number
  recordDate: string
  rates: RealmRates
  pvpType: 'open' | 'optional' | 'hardcore' | 'retro'
  createdAt: string
  features: string[]
}

export interface RealmTheme {
  primary: string
  secondary: string
  accent: string
  gradient: string
}

export interface RealmRates {
  experience: number
  skill: number
  loot: number
  magic: number
  spawn: number
}

// Guilds
export interface Guild {
  id: string
  name: string
  description: string
  motd: string
  realm: RealmId
  createdAt: string
  level: number
  logo?: string
  members: GuildMember[]
  ranks: GuildRank[]
  wars: GuildWar[]
  alliances: string[]
  enemies: string[]
}

export interface GuildMember {
  characterId: string
  characterName: string
  rank: string
  joinedAt: string
  vocation: Vocation
  level: number
  online: boolean
}

export interface GuildMembership {
  guildId: string
  guildName: string
  rank: string
}

export interface GuildRank {
  id: string
  name: string
  level: number
  permissions: string[]
}

export interface GuildWar {
  id: string
  opponentId: string
  opponentName: string
  status: 'pending' | 'active' | 'ended'
  startedAt: string
  endedAt?: string
  kills: number
  deaths: number
}

// Highscores
export interface HighscoreEntry {
  rank: number
  character: {
    id: string
    name: string
    vocation: Vocation
    level: number
    realm: RealmId
    online: boolean
  }
  value: number
  guild?: {
    id: string
    name: string
  }
}

export type HighscoreCategory = 
  | 'experience'
  | 'magic'
  | 'fist'
  | 'club'
  | 'sword'
  | 'axe'
  | 'distance'
  | 'shielding'
  | 'fishing'
  | 'achievements'
  | 'loyalty'
  | 'charm'

// Market
export interface MarketOffer {
  id: string
  type: 'buy' | 'sell'
  itemId: number
  itemName: string
  amount: number
  price: number
  character: {
    id: string
    name: string
  }
  realm: RealmId
  createdAt: string
  expiresAt: string
  anonymous: boolean
}

export interface MarketHistory {
  itemId: number
  itemName: string
  averagePrice: number
  minPrice: number
  maxPrice: number
  volume: number
  date: string
}

export interface Item {
  id: number
  name: string
  description: string
  type: ItemType
  attributes: ItemAttributes
  sprite: string
  tradeable: boolean
  stackable: boolean
}

export type ItemType = 
  | 'weapon'
  | 'armor'
  | 'helmet'
  | 'legs'
  | 'boots'
  | 'shield'
  | 'ring'
  | 'amulet'
  | 'container'
  | 'consumable'
  | 'rune'
  | 'tool'
  | 'decoration'
  | 'other'

export interface ItemAttributes {
  attack?: number
  defense?: number
  armor?: number
  charges?: number
  duration?: number
  levelRequired?: number
  vocationRequired?: Vocation[]
  weight: number
  slotType?: string
}

// News
export interface NewsArticle {
  id: string
  title: string
  slug: string
  excerpt: string
  content: string
  category: NewsCategory
  author: string
  publishedAt: string
  updatedAt?: string
  featured: boolean
  image?: string
  tags: string[]
  reactions: {
    likes: number
    comments: number
  }
}

export type NewsCategory = 
  | 'announcement'
  | 'update'
  | 'event'
  | 'community'
  | 'maintenance'
  | 'development'

// Forum
export interface ForumCategory {
  id: string
  name: string
  description: string
  slug: string
  icon: string
  color: string
  threads: number
  posts: number
  lastPost?: ForumPost
  subCategories?: ForumCategory[]
}

export interface ForumThread {
  id: string
  categoryId: string
  title: string
  slug: string
  author: ForumUser
  createdAt: string
  updatedAt: string
  views: number
  replies: number
  pinned: boolean
  locked: boolean
  hot: boolean
  posts: ForumPost[]
  lastPost?: ForumPost
  tags: string[]
}

export interface ForumPost {
  id: string
  threadId: string
  author: ForumUser
  content: string
  createdAt: string
  updatedAt?: string
  editedBy?: string
  reactions: PostReactions
  quotes?: string[]
  isDeleted: boolean
}

export interface ForumUser {
  id: string
  username: string
  avatar?: string
  title?: string
  posts: number
  reputation: number
  joinedAt: string
  isOnline: boolean
  badges: string[]
}

export interface PostReactions {
  likes: number
  helpful: number
  funny: number
  userReaction?: 'like' | 'helpful' | 'funny'
}

// Achievements
export interface Achievement {
  id: string
  name: string
  description: string
  category: AchievementCategory
  points: number
  secret: boolean
  icon: string
  rarity: 'common' | 'uncommon' | 'rare' | 'epic' | 'legendary'
  unlockedAt?: string
  progress?: {
    current: number
    required: number
  }
}

export type AchievementCategory =
  | 'exploration'
  | 'combat'
  | 'social'
  | 'economy'
  | 'collection'
  | 'skill'
  | 'quest'
  | 'special'

// NFT & Blockchain
export interface NFT {
  id: string
  tokenId: string
  chain: BlockchainChain
  contractAddress: string
  owner: string
  itemId?: number
  itemName?: string
  metadata: NFTMetadata
  mintedAt: string
  lastTransferAt?: string
}

export interface NFTMetadata {
  name: string
  description: string
  image: string
  animationUrl?: string
  externalUrl?: string
  attributes: NFTAttribute[]
}

export interface NFTAttribute {
  traitType: string
  value: string | number
  displayType?: 'number' | 'date' | 'boost_percentage'
}

export type BlockchainChain = 
  | 'ethereum'
  | 'polygon'
  | 'starknet'
  | 'bitcoin'
  | 'base'
  | 'arbitrum'

export interface WalletInfo {
  address: string
  chain: BlockchainChain
  connected: boolean
  balance?: string
  nfts: NFT[]
}

// Admin
export interface AdminStats {
  playersOnline: number
  playersTotal: number
  activeRealms: number
  ticketsOpen: number
  revenue24h: number
  newAccounts24h: number
}

export interface AdminLog {
  id: string
  type: 'info' | 'warning' | 'error' | 'critical'
  message: string
  details?: Record<string, unknown>
  timestamp: string
  source: string
}

export interface AdminAlert {
  id: string
  type: 'info' | 'warning' | 'error' | 'success'
  message: string
  timestamp: string
  acknowledged: boolean
}

export interface BanRecord {
  id: string
  accountId: string
  characterId?: string
  reason: string
  bannedBy: string
  bannedAt: string
  expiresAt?: string
  permanent: boolean
  appealStatus?: 'pending' | 'approved' | 'denied'
}

// Events
export interface GameEvent {
  id: string
  name: string
  description: string
  type: EventType
  startAt: string
  endAt: string
  realm?: RealmId
  rewards: EventReward[]
  status: 'scheduled' | 'active' | 'ended'
  participants?: number
}

export type EventType =
  | 'double_exp'
  | 'rapid_respawn'
  | 'world_boss'
  | 'invasion'
  | 'seasonal'
  | 'pvp_tournament'
  | 'community'

export interface EventReward {
  type: 'item' | 'experience' | 'coins' | 'achievement' | 'title'
  value: string | number
  chance?: number
}

// API Responses
export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  pageSize: number
  totalPages: number
}

export interface ApiError {
  code: string
  message: string
  details?: Record<string, string[]>
}

// WebSocket Events
export interface RealtimeEvent {
  type: RealtimeEventType
  payload: unknown
  timestamp: number
}

export type RealtimeEventType =
  | 'player.online'
  | 'player.offline'
  | 'player.levelup'
  | 'player.death'
  | 'player.achievement'
  | 'guild.message'
  | 'market.offer'
  | 'server.status'
  | 'server.broadcast'
  | 'event.start'
  | 'event.end'

// ============================================
// Spells & Magic
// ============================================
export type SpellElement = 'fire' | 'ice' | 'energy' | 'earth' | 'holy' | 'death' | 'physical' | 'healing'
export type SpellType = 'attack' | 'healing' | 'support' | 'summon'

export interface Spell {
  id: string
  name: string
  words: string
  element: SpellElement
  type: SpellType
  vocation: Vocation[]
  level: number
  mana: number
  cooldown: number
  groupCooldown?: number
  premium: boolean
  description: string
  damage?: { min: number; max: number }
  healing?: { min: number; max: number }
  area?: boolean
  rune?: boolean
  runeId?: number
  soul?: number
  reagents?: { itemId: number; amount: number }[]
}

// ============================================
// Kill Statistics
// ============================================
export interface KillStatistics {
  totalKills: number
  totalDeaths: number
  pvpKills: number
  pveKills: number
  bossKills: number
  avgKillsPerDay: number
  mostDangerousRealm: RealmId
  safestRealm: RealmId
}

export interface KillEntry {
  id: string
  killer: {
    id: string
    name: string
    level: number
    vocation: Vocation
    isPlayer: boolean
  }
  victim: {
    id: string
    name: string
    level: number
    vocation?: Vocation
    isPlayer: boolean
  }
  realm: RealmId
  type: 'pvp' | 'pve' | 'boss'
  timestamp: string
  location?: string
}

export interface TopKiller {
  rank: number
  character: {
    id: string
    name: string
    level: number
    vocation: Vocation
    realm: RealmId
    online: boolean
  }
  kills: number
  deaths: number
  kd: number
  guild?: {
    id: string
    name: string
  }
}

export interface BossHunter {
  character: {
    id: string
    name: string
    realm: RealmId
  }
  totalKills: number
  bosses: string[]
}

// ============================================
// Boosted Creatures & Bosses
// ============================================
export interface BoostedCreature {
  id: string
  name: string
  sprite: string
  lootBonus: number // percentage, e.g., 100 for +100%
  experienceBonus: number
  spawn: string
  difficulty: 'Easy' | 'Medium' | 'Hard' | 'Very Hard' | 'Extreme'
  resetsAt: string // ISO timestamp
}

export interface BoostedBoss {
  id: string
  name: string
  sprite: string
  lootBonus: number
  charmBonus: number
  spawn: string
  difficulty: 'Hard' | 'Very Hard' | 'Extreme' | 'Legendary'
  resetsAt: string
}

// ============================================
// World Quests
// ============================================
export interface WorldQuest {
  id: string
  name: string
  description: string
  realm: RealmId
  goal: number
  current: number
  progress: number // percentage
  reward: string
  endsAt: string
  status: 'active' | 'completed' | 'failed'
}

// ============================================
// Creatures & Monsters
// ============================================
export interface Creature {
  id: string
  name: string
  description: string
  sprite: string
  health: number
  experience: number
  armor: number
  speed: number
  summonable: boolean
  convincable: boolean
  illusionable: boolean
  race: CreatureRace
  difficulty: 'Easy' | 'Medium' | 'Hard' | 'Very Hard' | 'Extreme'
  immunities: SpellElement[]
  weaknesses: SpellElement[]
  resistances: { element: SpellElement; percent: number }[]
  loot: CreatureLoot[]
  spawnLocations: string[]
  bestiaryClass: string
  bestiaryOccurrence: 'Common' | 'Uncommon' | 'Rare' | 'Very Rare'
  charmPoints: number
}

export type CreatureRace = 
  | 'amphibic'
  | 'blood'
  | 'construct'
  | 'demon'
  | 'dragon'
  | 'elemental'
  | 'extra_dimensional'
  | 'fey'
  | 'giant'
  | 'human'
  | 'humanoid'
  | 'lycanthrope'
  | 'magical'
  | 'mammal'
  | 'plant'
  | 'reptile'
  | 'slime'
  | 'undead'
  | 'vermin'

export interface CreatureLoot {
  itemId: number
  itemName: string
  chance: number // percentage
  min: number
  max: number
}

// ============================================
// Calculator Types
// ============================================
export interface DamageCalculation {
  min: number
  max: number
  avg: number
  dps: number
  critical?: number
}

export interface ExperienceCalculation {
  expNeeded: number
  hoursNeeded: number
  daysNeeded: number
  staminaHours: number
  effectiveExpPerHour: number
}

export interface LootCalculation {
  totalKills: number
  grossLoot: number
  netProfit: number
  profitPerHour: number
}

