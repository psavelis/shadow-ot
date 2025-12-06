import { apiClient } from './client'
import {
  User,
  AuthTokens,
  LoginRequest,
  RegisterRequest,
  Character,
  CharacterDeath,
  CharacterKill,
  Realm,
  Guild,
  HighscoreEntry,
  HighscoreCategory,
  MarketOffer,
  MarketHistory,
  Item,
  NewsArticle,
  ForumCategory,
  ForumThread,
  ForumPost,
  Achievement,
  NFT,
  GameEvent,
  AdminStats,
  AdminLog,
  AdminAlert,
  BanRecord,
  PaginatedResponse,
  RealmId,
  Vocation,
  Spell,
  SpellElement,
  SpellType,
  KillStatistics,
  KillEntry,
  TopKiller,
  BossHunter,
  BoostedCreature,
  BoostedBoss,
  WorldQuest,
  Creature,
} from '../types'

// ============================================
// Authentication Endpoints
// ============================================
export const authApi = {
  login: (data: LoginRequest) =>
    apiClient.post<{ user: User; tokens: AuthTokens }>('/auth/login', data),

  register: (data: RegisterRequest) =>
    apiClient.post<{ user: User; tokens: AuthTokens }>('/auth/register', data),

  logout: () => apiClient.post('/auth/logout'),

  refreshToken: (data: { refreshToken: string }) =>
    apiClient.post<{ tokens: AuthTokens }>('/auth/refresh', data),

  // Password management
  requestPasswordReset: (data: { email: string }) =>
    apiClient.post('/auth/forgot-password', data),

  resetPassword: (data: { token: string; newPassword: string }) =>
    apiClient.post('/auth/reset-password', data),

  // Email verification
  verifyEmail: (data: { token: string }) =>
    apiClient.post('/auth/verify-email', data),

  resendVerification: () =>
    apiClient.post('/auth/resend-verification'),

  // Two-Factor Authentication
  enable2FA: () =>
    apiClient.post<{ secret: string; qrCode: string }>('/auth/2fa/enable'),

  verify2FA: (code: string) =>
    apiClient.post('/auth/2fa/verify', { code }),

  disable2FA: (code: string) =>
    apiClient.post('/auth/2fa/disable', { code }),

  // Wallet authentication (Web3)
  getWalletNonce: (address: string) =>
    apiClient.get<{ nonce: string }>(`/auth/wallet/nonce/${address}`),

  loginWithWallet: (data: { address: string; signature: string; chain: string }) =>
    apiClient.post<{ user: User; tokens: AuthTokens }>('/auth/wallet/login', data),

  connectWallet: (address: string, signature: string) =>
    apiClient.post('/auth/wallet/connect', { address, signature }),

  disconnectWallet: () =>
    apiClient.post('/auth/wallet/disconnect'),
}

// ============================================
// User/Account Endpoints
// ============================================
export const userApi = {
  getProfile: () =>
    apiClient.get<User>('/users/me'),

  updateProfile: (data: Partial<Pick<User, 'username' | 'email'>>) =>
    apiClient.patch<User>('/users/me', data),

  changePassword: (data: { currentPassword: string; newPassword: string }) =>
    apiClient.post('/users/me/password', data),

  getSettings: () =>
    apiClient.get<Record<string, unknown>>('/users/me/settings'),

  updateSettings: (settings: Record<string, unknown>) =>
    apiClient.patch('/users/me/settings', settings),

  // Session management
  getSessions: () =>
    apiClient.get<{ sessions: Array<{
      id: string
      device: string
      browser: string
      ip: string
      location: string
      lastActive: string
      current: boolean
    }> }>('/users/me/sessions'),

  revokeSession: (sessionId: string) =>
    apiClient.delete(`/users/me/sessions/${sessionId}`),

  revokeAllSessions: () =>
    apiClient.delete('/users/me/sessions'),

  // Two-Factor Authentication
  setup2FA: () =>
    apiClient.post<{ secret: string; qrCode: string; backupCodes: string[] }>('/users/me/2fa/setup'),

  enable2FA: (data: { secret: string; code: string }) =>
    apiClient.post('/users/me/2fa/enable', data),

  disable2FA: (data: { code: string; password: string }) =>
    apiClient.post('/users/me/2fa/disable', data),

  regenerateBackupCodes: (data: { code: string }) =>
    apiClient.post<{ backupCodes: string[] }>('/users/me/2fa/backup-codes', data),

  // Wallet management
  linkWallet: (data: { address: string; signature: string; chain: string }) =>
    apiClient.post('/users/me/wallets', data),

  unlinkWallet: (address: string) =>
    apiClient.delete(`/users/me/wallets/${address}`),

  // Account deletion
  requestAccountDeletion: (data: { password: string; reason?: string }) =>
    apiClient.post('/users/me/deletion/request', data),

  confirmAccountDeletion: (data: { token: string }) =>
    apiClient.post('/users/me/deletion/confirm', data),

  cancelAccountDeletion: () =>
    apiClient.post('/users/me/deletion/cancel'),

  // Activity Log
  getActivityLog: (params?: { limit?: number; page?: number }) =>
    apiClient.get<{
      logs: Array<{
        id: string
        action: string
        ip: string
        location: string
        userAgent: string
        timestamp: string
      }>
      total: number
    }>('/users/me/activity', params),

  // Security Keys (FIDO2/WebAuthn)
  getSecurityKeys: () =>
    apiClient.get<{
      keys: Array<{
        id: string
        name: string
        type: 'yubikey' | 'fido2'
        addedAt: string
        lastUsed: string
      }>
    }>('/users/me/security-keys'),

  challengeSecurityKey: () =>
    apiClient.post<{ options: PublicKeyCredentialCreationOptions }>('/users/me/security-keys/challenge'),

  registerSecurityKey: (data: { name: string; credential: unknown }) =>
    apiClient.post<{ id: string; name: string; type: string }>('/users/me/security-keys', data),

  deleteSecurityKey: (id: string) =>
    apiClient.delete(`/users/me/security-keys/${id}`),

  // SSO Settings
  getSSOStatus: () =>
    apiClient.get<{
      enabled: boolean
      realms: Array<{
        realmId: string
        realmName: string
        enabled: boolean
        lastSync: string | null
      }>
    }>('/users/me/sso'),

  toggleSSO: (enabled: boolean) =>
    apiClient.patch('/users/me/sso', { enabled }),

  toggleSSOForRealm: (realmId: string, enabled: boolean) =>
    apiClient.patch(`/users/me/sso/realms/${realmId}`, { enabled }),

  // Transactions
  getTransactions: (params?: { 
    type?: 'market' | 'transfer' | 'nft' | 'premium'
    page?: number
    pageSize?: number 
  }) => apiClient.get<PaginatedResponse<Transaction>>('/users/me/transactions', params),

  // Notifications
  getNotifications: (params?: { 
    type?: 'levelup' | 'trade' | 'achievement' | 'guild' | 'system'
    unreadOnly?: boolean
    page?: number
    pageSize?: number 
  }) => apiClient.get<PaginatedResponse<Notification>>('/users/me/notifications', params),

  markNotificationRead: (id: string) =>
    apiClient.patch(`/users/me/notifications/${id}/read`),

  markAllNotificationsRead: () =>
    apiClient.post('/users/me/notifications/read-all'),

  deleteNotification: (id: string) =>
    apiClient.delete(`/users/me/notifications/${id}`),

  // Premium
  getPremiumStatus: () =>
    apiClient.get<{
      active: boolean
      plan: 'monthly' | 'quarterly' | 'yearly' | null
      expiresAt: string | null
      coins: number
    }>('/users/me/premium'),

  purchasePremium: (plan: 'monthly' | 'quarterly' | 'yearly') =>
    apiClient.post('/users/me/premium/purchase', { plan }),

  purchaseCoins: (packageId: number) =>
    apiClient.post('/users/me/premium/coins', { packageId }),

  getPremiumHistory: () =>
    apiClient.get<Array<{
      id: string
      type: 'subscription' | 'coins'
      description: string
      amount: number
      date: string
      status: 'completed' | 'pending' | 'failed'
    }>>('/users/me/premium/history'),
}

// ============================================
// Transaction Type
// ============================================
export interface Transaction {
  id: string
  type: 'market' | 'transfer' | 'nft' | 'premium'
  title: string
  description: string
  amount: number
  currency: 'gold' | 'coins' | 'usd'
  timestamp: string
  status: 'completed' | 'pending' | 'failed'
  from?: string
  to?: string
  itemId?: string
}

// ============================================
// Notification Type
// ============================================
export interface Notification {
  id: string
  type: 'levelup' | 'trade' | 'achievement' | 'guild' | 'system'
  title: string
  message: string
  timestamp: string
  read: boolean
  actionUrl?: string
  data?: Record<string, unknown>
}

// ============================================
// Inventory Endpoints
// ============================================
export const inventoryApi = {
  getItems: (params?: {
    characterId?: string
    category?: string
    search?: string
    page?: number
    pageSize?: number
  }) => apiClient.get<PaginatedResponse<InventoryItem>>('/inventory', params),

  getItem: (id: string) =>
    apiClient.get<InventoryItem>(`/inventory/${id}`),

  transferItem: (itemId: string, toCharacterId: string) =>
    apiClient.post(`/inventory/${itemId}/transfer`, { toCharacterId }),

  listOnMarket: (itemId: string, price: number) =>
    apiClient.post(`/inventory/${itemId}/list`, { price }),
}

export interface InventoryItem {
  id: string
  name: string
  type: 'weapon' | 'armor' | 'helmet' | 'legs' | 'boots' | 'shield' | 'amulet' | 'ring' | 'consumable' | 'tool' | 'quest'
  rarity: 'common' | 'uncommon' | 'rare' | 'epic' | 'legendary'
  quantity: number
  attributes: Record<string, number>
  imbuements?: string[]
  spriteId: number
  equipped: boolean
  characterId: string
  description?: string
  requirements?: {
    level?: number
    vocation?: string[]
  }
}

// ============================================
// Character Endpoints
// ============================================
export const characterApi = {
  getAll: () =>
    apiClient.get<Character[]>('/characters'),

  getById: (id: string) =>
    apiClient.get<Character>(`/characters/${id}`),

  getByName: (name: string) =>
    apiClient.get<Character>(`/characters/name/${name}`),

  create: (data: { name: string; vocation: Vocation; sex: 'male' | 'female'; realm: RealmId }) =>
    apiClient.post<Character>('/characters', data),

  delete: (id: string) =>
    apiClient.delete(`/characters/${id}`),

  rename: (id: string, newName: string) =>
    apiClient.patch<Character>(`/characters/${id}/rename`, { name: newName }),

  transfer: (id: string, targetRealm: RealmId) =>
    apiClient.post(`/characters/${id}/transfer`, { realm: targetRealm }),

  getDeaths: (id: string, page = 1, pageSize = 20) =>
    apiClient.get<CharacterDeath[]>(`/characters/${id}/deaths`, { page, pageSize }),

  getKills: (id: string, page = 1, pageSize = 20) =>
    apiClient.get<CharacterKill[]>(`/characters/${id}/kills`, { page, pageSize }),

  getAchievements: (id: string) =>
    apiClient.get<Achievement[]>(`/characters/${id}/achievements`),

  updateOutfit: (id: string, outfit: Character['outfit']) =>
    apiClient.patch(`/characters/${id}/outfit`, outfit),
}

// ============================================
// Realm Endpoints
// ============================================
export const realmApi = {
  getAll: () =>
    apiClient.get<Realm[]>('/realms'),

  getById: (id: RealmId) =>
    apiClient.get<Realm>(`/realms/${id}`),

  getOnlinePlayers: (id: RealmId) =>
    apiClient.get<{ name: string; level: number; vocation: Vocation }[]>(`/realms/${id}/online`),

  getStatus: (id: RealmId) =>
    apiClient.get<{ status: Realm['status']; playersOnline: number; uptime: number }>(`/realms/${id}/status`),
}

// ============================================
// Highscore Endpoints
// ============================================
export const highscoreApi = {
  get: (params: {
    category?: HighscoreCategory
    realm?: RealmId
    vocation?: Vocation
    page?: number
    pageSize?: number
  }) => apiClient.get<PaginatedResponse<HighscoreEntry>>('/highscores', params),

  getTop: (category: HighscoreCategory, limit = 10) =>
    apiClient.get<HighscoreEntry[]>(`/highscores/top/${category}`, { limit }),

  getCharacterRank: (characterId: string, category: HighscoreCategory) =>
    apiClient.get<{ rank: number; value: number }>(`/highscores/rank/${characterId}/${category}`),
}

// ============================================
// Guild Endpoints
// ============================================
export const guildApi = {
  getAll: (params: { realm?: RealmId; page?: number; pageSize?: number }) =>
    apiClient.get<PaginatedResponse<Guild>>('/guilds', params),

  getById: (id: string) =>
    apiClient.get<Guild>(`/guilds/${id}`),

  getByName: (name: string) =>
    apiClient.get<Guild>(`/guilds/name/${name}`),

  create: (data: { name: string; realm: RealmId }) =>
    apiClient.post<Guild>('/guilds', data),

  update: (id: string, data: Partial<Pick<Guild, 'description' | 'motd'>>) =>
    apiClient.patch<Guild>(`/guilds/${id}`, data),

  delete: (id: string) =>
    apiClient.delete(`/guilds/${id}`),

  invite: (guildId: string, characterName: string) =>
    apiClient.post(`/guilds/${guildId}/invite`, { characterName }),

  kick: (guildId: string, characterId: string) =>
    apiClient.post(`/guilds/${guildId}/kick`, { characterId }),

  leave: (guildId: string) =>
    apiClient.post(`/guilds/${guildId}/leave`),

  setRank: (guildId: string, characterId: string, rank: string) =>
    apiClient.patch(`/guilds/${guildId}/members/${characterId}/rank`, { rank }),

  declareWar: (guildId: string, targetGuildId: string) =>
    apiClient.post(`/guilds/${guildId}/war/declare`, { targetGuildId }),

  surrenderWar: (guildId: string, warId: string) =>
    apiClient.post(`/guilds/${guildId}/war/${warId}/surrender`),
}

// ============================================
// Market Endpoints
// ============================================
export const marketApi = {
  getOffers: (params: {
    type?: 'buy' | 'sell'
    itemId?: number
    realm?: RealmId
    page?: number
    pageSize?: number
  }) => apiClient.get<PaginatedResponse<MarketOffer>>('/market/offers', params),

  createOffer: (data: {
    type: 'buy' | 'sell'
    itemId: number
    amount: number
    price: number
    anonymous?: boolean
  }) => apiClient.post<MarketOffer>('/market/offers', data),

  cancelOffer: (id: string) =>
    apiClient.delete(`/market/offers/${id}`),

  acceptOffer: (id: string, amount: number) =>
    apiClient.post(`/market/offers/${id}/accept`, { amount }),

  getHistory: (itemId: number, days = 30) =>
    apiClient.get<MarketHistory[]>(`/market/history/${itemId}`, { days }),

  getItems: (params: { search?: string; type?: string }) =>
    apiClient.get<Item[]>('/market/items', params),

  getItemById: (id: number) =>
    apiClient.get<Item>(`/market/items/${id}`),
}

// ============================================
// News Endpoints
// ============================================
export const newsApi = {
  getAll: (params: { category?: string; page?: number; pageSize?: number }) =>
    apiClient.get<PaginatedResponse<NewsArticle>>('/news', params),

  getFeatured: () =>
    apiClient.get<NewsArticle[]>('/news/featured'),

  getBySlug: (slug: string) =>
    apiClient.get<NewsArticle>(`/news/${slug}`),

  react: (id: string, type: 'like') =>
    apiClient.post(`/news/${id}/react`, { type }),
}

// ============================================
// Forum Endpoints
// ============================================
export const forumApi = {
  getCategories: () =>
    apiClient.get<ForumCategory[]>('/forum/categories'),

  getCategoryBySlug: (slug: string) =>
    apiClient.get<ForumCategory>(`/forum/categories/${slug}`),

  getThreads: (params: {
    categoryId?: string
    page?: number
    pageSize?: number
    sort?: 'latest' | 'popular' | 'oldest'
  }) => apiClient.get<PaginatedResponse<ForumThread>>('/forum/threads', params),

  getThreadBySlug: (slug: string) =>
    apiClient.get<ForumThread>(`/forum/threads/${slug}`),

  createThread: (data: { categoryId: string; title: string; content: string; tags?: string[] }) =>
    apiClient.post<ForumThread>('/forum/threads', data),

  updateThread: (id: string, data: Partial<Pick<ForumThread, 'title' | 'pinned' | 'locked'>>) =>
    apiClient.patch<ForumThread>(`/forum/threads/${id}`, data),

  deleteThread: (id: string) =>
    apiClient.delete(`/forum/threads/${id}`),

  getPosts: (threadId: string, params: { page?: number; pageSize?: number }) =>
    apiClient.get<PaginatedResponse<ForumPost>>(`/forum/threads/${threadId}/posts`, params),

  createPost: (threadId: string, content: string, quoteIds?: string[]) =>
    apiClient.post<ForumPost>(`/forum/threads/${threadId}/posts`, { content, quoteIds }),

  updatePost: (postId: string, content: string) =>
    apiClient.patch<ForumPost>(`/forum/posts/${postId}`, { content }),

  deletePost: (postId: string) =>
    apiClient.delete(`/forum/posts/${postId}`),

  reactToPost: (postId: string, reaction: 'like' | 'helpful' | 'funny') =>
    apiClient.post(`/forum/posts/${postId}/react`, { reaction }),

  search: (query: string, params: { page?: number; pageSize?: number }) =>
    apiClient.get<PaginatedResponse<ForumThread>>('/forum/search', { query, ...params }),
}

// ============================================
// Achievement Endpoints
// ============================================
export const achievementApi = {
  getAll: () =>
    apiClient.get<Achievement[]>('/achievements'),

  getByCategory: (category: string) =>
    apiClient.get<Achievement[]>(`/achievements/category/${category}`),

  getProgress: (characterId: string) =>
    apiClient.get<Achievement[]>(`/characters/${characterId}/achievements`),
}

// ============================================
// NFT/Blockchain Endpoints
// ============================================
export const nftApi = {
  getOwned: () =>
    apiClient.get<NFT[]>('/nft/owned'),

  getByTokenId: (chain: string, tokenId: string) =>
    apiClient.get<NFT>(`/nft/${chain}/${tokenId}`),

  mint: (itemId: number, chain: string) =>
    apiClient.post<{ txHash: string; nft: NFT }>('/nft/mint', { itemId, chain }),

  transfer: (nftId: string, toAddress: string) =>
    apiClient.post('/nft/transfer', { nftId, toAddress }),

  getMarketplace: (params: { chain?: string; page?: number; pageSize?: number }) =>
    apiClient.get<PaginatedResponse<NFT & { price: string; seller: string }>>('/nft/marketplace', params),

  listForSale: (nftId: string, price: string) =>
    apiClient.post('/nft/list', { nftId, price }),

  buyNft: (nftId: string) =>
    apiClient.post('/nft/buy', { nftId }),

  cancelListing: (nftId: string) =>
    apiClient.post('/nft/cancel-listing', { nftId }),
}

// ============================================
// Event Endpoints
// ============================================
export const eventApi = {
  getAll: (params: { status?: GameEvent['status']; realm?: RealmId }) =>
    apiClient.get<GameEvent[]>('/events', params),

  getActive: () =>
    apiClient.get<GameEvent[]>('/events/active'),

  getUpcoming: () =>
    apiClient.get<GameEvent[]>('/events/upcoming'),

  getById: (id: string) =>
    apiClient.get<GameEvent>(`/events/${id}`),
}

// ============================================
// Admin Endpoints
// ============================================
export const adminApi = {
  getStats: () =>
    apiClient.get<AdminStats>('/admin/stats'),

  getLogs: (params: { type?: AdminLog['type']; page?: number; pageSize?: number }) =>
    apiClient.get<PaginatedResponse<AdminLog>>('/admin/logs', params),

  getAlerts: () =>
    apiClient.get<AdminAlert[]>('/admin/alerts'),

  acknowledgeAlert: (id: string) =>
    apiClient.patch(`/admin/alerts/${id}/acknowledge`),

  // Player management
  searchPlayers: (query: string) =>
    apiClient.get<User[]>('/admin/players/search', { query }),

  getPlayerDetails: (id: string) =>
    apiClient.get<User & { characters: Character[] }>(`/admin/players/${id}`),

  banPlayer: (data: {
    accountId: string
    characterId?: string
    reason: string
    duration?: number // minutes, null for permanent
  }) => apiClient.post<BanRecord>('/admin/players/ban', data),

  unbanPlayer: (banId: string) =>
    apiClient.delete(`/admin/players/ban/${banId}`),

  getBans: (params: { page?: number; pageSize?: number }) =>
    apiClient.get<PaginatedResponse<BanRecord>>('/admin/bans', params),

  warnPlayer: (accountId: string, reason: string) =>
    apiClient.post('/admin/players/warn', { accountId, reason }),

  // Server management
  getRealmStatus: (realmId: RealmId) =>
    apiClient.get<{ cpu: number; memory: number; uptime: number; connections: number }>(`/admin/realms/${realmId}/status`),

  restartRealm: (realmId: RealmId) =>
    apiClient.post(`/admin/realms/${realmId}/restart`),

  broadcast: (message: string, realm?: RealmId) =>
    apiClient.post('/admin/broadcast', { message, realm }),

  // Event management
  createEvent: (data: Omit<GameEvent, 'id' | 'status'>) =>
    apiClient.post<GameEvent>('/admin/events', data),

  updateEvent: (id: string, data: Partial<GameEvent>) =>
    apiClient.patch<GameEvent>(`/admin/events/${id}`, data),

  deleteEvent: (id: string) =>
    apiClient.delete(`/admin/events/${id}`),

  // News management
  createNews: (data: Omit<NewsArticle, 'id' | 'publishedAt' | 'reactions'>) =>
    apiClient.post<NewsArticle>('/admin/news', data),

  updateNews: (id: string, data: Partial<NewsArticle>) =>
    apiClient.patch<NewsArticle>(`/admin/news/${id}`, data),

  deleteNews: (id: string) =>
    apiClient.delete(`/admin/news/${id}`),
}

// ============================================
// Spell Endpoints
// ============================================
export const spellApi = {
  getAll: (params?: {
    element?: SpellElement
    type?: SpellType
    vocation?: Vocation
    premium?: boolean
    search?: string
  }) => apiClient.get<Spell[]>('/spells', params),

  getById: (id: string) =>
    apiClient.get<Spell>(`/spells/${id}`),

  getByWords: (words: string) =>
    apiClient.get<Spell>(`/spells/words/${encodeURIComponent(words)}`),

  getByVocation: (vocation: Vocation) =>
    apiClient.get<Spell[]>(`/spells/vocation/${vocation}`),

  getByElement: (element: SpellElement) =>
    apiClient.get<Spell[]>(`/spells/element/${element}`),

  getRunes: () =>
    apiClient.get<Spell[]>('/spells/runes'),
}

// ============================================
// Kill Statistics Endpoints
// ============================================
export const killStatsApi = {
  getStatistics: (realm?: RealmId) =>
    apiClient.get<KillStatistics>('/kill-statistics', { realm }),

  getTopKillers: (params?: {
    realm?: RealmId
    type?: 'pvp' | 'pve' | 'boss' | 'all'
    timeRange?: 'today' | 'week' | 'month' | 'all'
    limit?: number
  }) => apiClient.get<TopKiller[]>('/kill-statistics/top-killers', params),

  getRecentDeaths: (params?: {
    realm?: RealmId
    type?: 'pvp' | 'pve' | 'boss' | 'all'
    limit?: number
  }) => apiClient.get<KillEntry[]>('/kill-statistics/recent', params),

  getBossHunters: (params?: {
    realm?: RealmId
    limit?: number
  }) => apiClient.get<BossHunter[]>('/kill-statistics/boss-hunters', params),

  getCharacterKills: (characterId: string, params?: {
    type?: 'pvp' | 'pve' | 'boss' | 'all'
    page?: number
    pageSize?: number
  }) => apiClient.get<PaginatedResponse<KillEntry>>(`/kill-statistics/character/${characterId}`, params),
}

// ============================================
// Boosted Creature/Boss Endpoints
// ============================================
export const boostedApi = {
  getCreature: () =>
    apiClient.get<BoostedCreature>('/boosted/creature'),

  getBoss: () =>
    apiClient.get<BoostedBoss>('/boosted/boss'),

  getHistory: (type: 'creature' | 'boss', days?: number) =>
    apiClient.get<(BoostedCreature | BoostedBoss)[]>(`/boosted/${type}/history`, { days }),
}

// ============================================
// World Quest Endpoints
// ============================================
export const worldQuestApi = {
  getAll: (params?: { realm?: RealmId; status?: 'active' | 'completed' | 'failed' }) =>
    apiClient.get<WorldQuest[]>('/world-quests', params),

  getActive: (realm?: RealmId) =>
    apiClient.get<WorldQuest[]>('/world-quests/active', { realm }),

  getById: (id: string) =>
    apiClient.get<WorldQuest>(`/world-quests/${id}`),

  contribute: (id: string, amount: number) =>
    apiClient.post(`/world-quests/${id}/contribute`, { amount }),
}

// ============================================
// Creature/Bestiary Endpoints
// ============================================
export const creatureApi = {
  getAll: (params?: {
    race?: string
    difficulty?: string
    search?: string
    page?: number
    pageSize?: number
  }) => apiClient.get<PaginatedResponse<Creature>>('/creatures', params),

  getById: (id: string) =>
    apiClient.get<Creature>(`/creatures/${id}`),

  getByName: (name: string) =>
    apiClient.get<Creature>(`/creatures/name/${encodeURIComponent(name)}`),

  getBestiaryProgress: (characterId: string) =>
    apiClient.get<{
      creature: Creature
      kills: number
      stage: 1 | 2 | 3 | 4
      completed: boolean
    }[]>(`/characters/${characterId}/bestiary`),

  getBestiaryEntry: (characterId: string, creatureId: string) =>
    apiClient.get<{
      creature: Creature
      kills: number
      stage: 1 | 2 | 3 | 4
      completed: boolean
      unlockedLoot: boolean
      unlockedCharm: boolean
    }>(`/characters/${characterId}/bestiary/${creatureId}`),
}

// ============================================
// OAuth Endpoints
// ============================================
export const authApi = {
  ...{
    login: (data: LoginRequest) =>
      apiClient.post<{ user: User; tokens: AuthTokens }>('/auth/login', data),

    register: (data: RegisterRequest) =>
      apiClient.post<{ user: User; tokens: AuthTokens }>('/auth/register', data),

    logout: () => apiClient.post('/auth/logout'),

    refreshToken: (data: { refreshToken: string }) =>
      apiClient.post<{ tokens: AuthTokens }>('/auth/refresh', data),

    requestPasswordReset: (data: { email: string }) =>
      apiClient.post('/auth/forgot-password', data),

    resetPassword: (data: { token: string; newPassword: string }) =>
      apiClient.post('/auth/reset-password', data),

    verifyEmail: (data: { token: string }) =>
      apiClient.post('/auth/verify-email', data),

    resendVerification: () =>
      apiClient.post('/auth/resend-verification'),

    enable2FA: () =>
      apiClient.post<{ secret: string; qrCode: string }>('/auth/2fa/enable'),

    verify2FA: (code: string) =>
      apiClient.post('/auth/2fa/verify', { code }),

    disable2FA: (code: string) =>
      apiClient.post('/auth/2fa/disable', { code }),

    getWalletNonce: (address: string) =>
      apiClient.get<{ nonce: string }>(`/auth/wallet/nonce/${address}`),

    loginWithWallet: (data: { address: string; signature: string; chain: string }) =>
      apiClient.post<{ user: User; tokens: AuthTokens }>('/auth/wallet/login', data),

    connectWallet: (address: string, signature: string) =>
      apiClient.post('/auth/wallet/connect', { address, signature }),

    disconnectWallet: () =>
      apiClient.post('/auth/wallet/disconnect'),
  },

  // OAuth providers
  oauthCallback: (data: { provider: string; code: string; state: string }) =>
    apiClient.post<{ user: User; tokens: AuthTokens }>('/auth/oauth/callback', data),

  linkOAuth: (provider: string) =>
    apiClient.post<{ redirectUrl: string }>(`/auth/oauth/${provider}/link`),

  unlinkOAuth: (provider: string) =>
    apiClient.post(`/auth/oauth/${provider}/unlink`),

  getLinkedAccounts: () =>
    apiClient.get<{
      google?: { email: string; linkedAt: string }
      discord?: { username: string; linkedAt: string }
      twitch?: { username: string; linkedAt: string }
    }>('/auth/oauth/linked'),

  // Security Keys (FIDO2/WebAuthn)
  registerSecurityKey: (data: { name: string; credential: unknown }) =>
    apiClient.post<{ id: string; name: string; type: string; createdAt: string }>('/auth/security-keys/register', data),

  getSecurityKeys: () =>
    apiClient.get<{
      id: string
      name: string
      type: 'yubikey' | 'fido2'
      addedAt: string
      lastUsed: string
    }[]>('/auth/security-keys'),

  deleteSecurityKey: (id: string) =>
    apiClient.delete(`/auth/security-keys/${id}`),

  challengeSecurityKey: () =>
    apiClient.post<{ challenge: string; allowCredentials: unknown[] }>('/auth/security-keys/challenge'),

  verifySecurityKey: (data: { credential: unknown }) =>
    apiClient.post<{ user: User; tokens: AuthTokens }>('/auth/security-keys/verify', data),

  // SSO
  getSsoSettings: () =>
    apiClient.get<{
      enabled: boolean
      realms: { id: RealmId; enabled: boolean; lastSync: string }[]
    }>('/auth/sso'),

  updateSsoSettings: (data: { enabled: boolean; realms: { id: RealmId; enabled: boolean }[] }) =>
    apiClient.patch('/auth/sso', data),
}

// ============================================
// Support Ticket Endpoints
// ============================================
export interface SupportTicket {
  id: string
  subject: string
  category: 'technical' | 'billing' | 'account' | 'report' | 'other'
  status: 'open' | 'pending' | 'resolved' | 'closed'
  priority: 'low' | 'medium' | 'high' | 'urgent'
  createdAt: string
  updatedAt: string
  messages: Array<{
    id: string
    content: string
    author: 'user' | 'support'
    createdAt: string
    attachments?: string[]
  }>
}

export const supportApi = {
  getTickets: (params?: { status?: SupportTicket['status']; page?: number; pageSize?: number }) =>
    apiClient.get<PaginatedResponse<SupportTicket>>('/support/tickets', params),

  getTicket: (id: string) =>
    apiClient.get<SupportTicket>(`/support/tickets/${id}`),

  createTicket: (data: { subject: string; category: SupportTicket['category']; message: string }) =>
    apiClient.post<SupportTicket>('/support/tickets', data),

  replyToTicket: (id: string, message: string) =>
    apiClient.post<SupportTicket>(`/support/tickets/${id}/reply`, { message }),

  closeTicket: (id: string) =>
    apiClient.patch(`/support/tickets/${id}/close`),

  getFAQ: () =>
    apiClient.get<Array<{
      category: string
      items: Array<{ question: string; answer: string }>
    }>>('/support/faq'),
}

// ============================================
// Auction Endpoints
// ============================================
export interface CharacterAuction {
  id: string
  characterId: string
  characterName: string
  level: number
  vocation: Vocation
  skills: Record<string, number>
  currentBid: number
  minimumBid: number
  buyoutPrice?: number
  seller: { id: string; name: string }
  highestBidder?: { id: string; name: string }
  startTime: string
  endTime: string
  status: 'active' | 'ended' | 'cancelled'
  bidCount: number
}

export interface ItemAuction {
  id: string
  itemId: string
  itemName: string
  itemType: string
  rarity: 'common' | 'uncommon' | 'rare' | 'epic' | 'legendary'
  quantity: number
  attributes: Record<string, number>
  currentBid: number
  minimumBid: number
  buyoutPrice?: number
  seller: { id: string; name: string }
  highestBidder?: { id: string; name: string }
  startTime: string
  endTime: string
  status: 'active' | 'ended' | 'cancelled'
  bidCount: number
  spriteId: number
}

export const auctionApi = {
  getCharacterAuctions: (params?: {
    vocation?: Vocation
    minLevel?: number
    maxLevel?: number
    sort?: 'ending_soon' | 'price_asc' | 'price_desc' | 'level'
    page?: number
    pageSize?: number
  }) => apiClient.get<PaginatedResponse<CharacterAuction>>('/auctions/characters', params),

  getItemAuctions: (params?: {
    type?: string
    rarity?: string
    sort?: 'ending_soon' | 'price_asc' | 'price_desc'
    page?: number
    pageSize?: number
  }) => apiClient.get<PaginatedResponse<ItemAuction>>('/auctions/items', params),

  getAuction: (id: string) =>
    apiClient.get<CharacterAuction | ItemAuction>(`/auctions/${id}`),

  placeBid: (id: string, amount: number) =>
    apiClient.post(`/auctions/${id}/bid`, { amount }),

  buyout: (id: string) =>
    apiClient.post(`/auctions/${id}/buyout`),

  getMyBids: () =>
    apiClient.get<Array<{
      auctionId: string
      type: 'character' | 'item'
      name: string
      myBid: number
      currentBid: number
      status: 'winning' | 'outbid' | 'won' | 'lost'
      endTime: string
    }>>('/auctions/my-bids'),

  createCharacterAuction: (data: {
    characterId: string
    minimumBid: number
    buyoutPrice?: number
    duration: number // hours
  }) => apiClient.post<CharacterAuction>('/auctions/characters', data),

  createItemAuction: (data: {
    itemId: string
    quantity: number
    minimumBid: number
    buyoutPrice?: number
    duration: number
  }) => apiClient.post<ItemAuction>('/auctions/items', data),
}

// ============================================
// House Endpoints
// ============================================
export interface House {
  id: string
  name: string
  town: string
  type: 'house' | 'guildhall'
  size: 'small' | 'medium' | 'large' | 'extra_large'
  beds: number
  sqm: number
  rent: number
  status: 'available' | 'auction' | 'rented'
  owner?: { id: string; name: string; since: string }
  auction?: {
    currentBid: number
    highestBidder?: { id: string; name: string }
    endTime: string
  }
  position: { x: number; y: number; z: number }
  description?: string
}

export const houseApi = {
  getHouses: (params?: {
    town?: string
    type?: House['type']
    size?: House['size']
    status?: House['status']
    minRent?: number
    maxRent?: number
    page?: number
    pageSize?: number
  }) => apiClient.get<PaginatedResponse<House>>('/houses', params),

  getHouse: (id: string) =>
    apiClient.get<House>(`/houses/${id}`),

  getMyHouses: () =>
    apiClient.get<Array<House & { character: string; paidUntil: string }>>('/houses/mine'),

  bidOnHouse: (id: string, amount: number) =>
    apiClient.post(`/houses/${id}/bid`, { amount }),

  leaveHouse: (id: string) =>
    apiClient.post(`/houses/${id}/leave`),

  transferHouse: (id: string, toCharacterId: string) =>
    apiClient.post(`/houses/${id}/transfer`, { toCharacterId }),

  payRent: (id: string) =>
    apiClient.post(`/houses/${id}/pay-rent`),
}

// ============================================
// Achievement Endpoints
// ============================================
export interface Achievement {
  id: string
  name: string
  description: string
  category: 'exploration' | 'combat' | 'social' | 'economy' | 'collection' | 'special'
  points: number
  rarity: 'common' | 'uncommon' | 'rare' | 'epic' | 'legendary'
  secret: boolean
  icon?: string
  requirements?: string
}

export interface PlayerAchievement extends Achievement {
  unlocked: boolean
  unlockedAt?: string
  progress?: { current: number; required: number }
}

export const achievementApi = {
  getAll: (params?: { category?: Achievement['category']; page?: number; pageSize?: number }) =>
    apiClient.get<PaginatedResponse<Achievement>>('/achievements', params),

  getPlayerAchievements: (characterId?: string) =>
    apiClient.get<{
      achievements: PlayerAchievement[]
      totalPoints: number
      completedCount: number
      totalCount: number
    }>('/achievements/player', characterId ? { characterId } : undefined),

  getLeaderboard: (params?: { page?: number; pageSize?: number }) =>
    apiClient.get<PaginatedResponse<{
      rank: number
      character: { id: string; name: string; level: number; vocation: Vocation }
      points: number
      completedCount: number
    }>>('/achievements/leaderboard', params),
}

