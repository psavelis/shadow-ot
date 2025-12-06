//! Game event system for server-wide event broadcasting and handling

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{CharacterId, PlayerId, RealmId};

/// Game-wide events that can be broadcast to all interested parties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    // Player events
    PlayerLogin(PlayerLoginEvent),
    PlayerLogout(PlayerLogoutEvent),
    PlayerDeath(PlayerDeathEvent),
    PlayerLevelUp(PlayerLevelUpEvent),
    PlayerSkillUp(PlayerSkillUpEvent),

    // Combat events
    CombatDamage(CombatDamageEvent),
    CreatureKill(CreatureKillEvent),
    PvPKill(PvPKillEvent),

    // World events
    ServerMessage(ServerMessageEvent),
    GlobalBroadcast(GlobalBroadcastEvent),
    SeasonalEventStart(SeasonalEventEvent),
    SeasonalEventEnd(SeasonalEventEvent),
    WorldBossSpawn(WorldBossEvent),

    // Economy events
    MarketTransaction(MarketTransactionEvent),
    CrossRealmTrade(CrossRealmTradeEvent),

    // Social events
    GuildCreated(GuildEvent),
    GuildDisbanded(GuildEvent),
    GuildWar(GuildWarEvent),
    PartyFormed(PartyEvent),

    // House events
    HousePurchased(HouseEvent),
    HouseTransferred(HouseEvent),

    // Achievement events
    AchievementUnlocked(AchievementEvent),
    BestiaryCompleted(BestiaryEvent),

    // Admin events
    AdminAction(AdminActionEvent),
    ServerMaintenance(MaintenanceEvent),
    RealmStatusChange(RealmStatusEvent),

    // Matchmaking events
    MatchFound(MatchmakingEvent),
    TournamentStarted(TournamentEvent),
    TournamentEnded(TournamentEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLoginEvent {
    pub player_id: PlayerId,
    pub character_id: CharacterId,
    pub character_name: String,
    pub realm_id: RealmId,
    pub ip_address: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLogoutEvent {
    pub player_id: PlayerId,
    pub character_id: CharacterId,
    pub character_name: String,
    pub realm_id: RealmId,
    pub session_duration_seconds: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerDeathEvent {
    pub victim_id: CharacterId,
    pub victim_name: String,
    pub killer_id: Option<CharacterId>,
    pub killer_name: Option<String>,
    pub killer_creature: Option<String>,
    pub position: Position,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLevelUpEvent {
    pub character_id: CharacterId,
    pub character_name: String,
    pub old_level: u16,
    pub new_level: u16,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSkillUpEvent {
    pub character_id: CharacterId,
    pub character_name: String,
    pub skill: Skill,
    pub old_level: u16,
    pub new_level: u16,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatDamageEvent {
    pub attacker_id: Uuid,
    pub target_id: Uuid,
    pub damage: i32,
    pub damage_type: DamageType,
    pub is_critical: bool,
    pub position: Position,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureKillEvent {
    pub killer_id: CharacterId,
    pub killer_name: String,
    pub creature_name: String,
    pub creature_id: u32,
    pub experience_gained: u64,
    pub loot: Vec<LootItem>,
    pub position: Position,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PvPKillEvent {
    pub killer_id: CharacterId,
    pub killer_name: String,
    pub victim_id: CharacterId,
    pub victim_name: String,
    pub was_justified: bool,
    pub skull_type: Option<SkullType>,
    pub position: Position,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMessageEvent {
    pub message: String,
    pub message_type: ServerMessageType,
    pub target_realm: Option<RealmId>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalBroadcastEvent {
    pub message: String,
    pub broadcast_type: BroadcastType,
    pub sender: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalEventEvent {
    pub event_id: Uuid,
    pub event_name: String,
    pub event_type: SeasonalEventType,
    pub affected_realms: Vec<RealmId>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldBossEvent {
    pub boss_name: String,
    pub boss_id: u32,
    pub spawn_position: Position,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTransactionEvent {
    pub seller_id: CharacterId,
    pub buyer_id: CharacterId,
    pub item_id: u32,
    pub item_name: String,
    pub quantity: u32,
    pub price: u64,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossRealmTradeEvent {
    pub seller_id: CharacterId,
    pub seller_realm: RealmId,
    pub buyer_id: CharacterId,
    pub buyer_realm: RealmId,
    pub item_id: u32,
    pub item_name: String,
    pub quantity: u32,
    pub price: u64,
    pub conversion_rate: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildEvent {
    pub guild_id: Uuid,
    pub guild_name: String,
    pub leader_id: CharacterId,
    pub leader_name: String,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildWarEvent {
    pub guild_a_id: Uuid,
    pub guild_a_name: String,
    pub guild_b_id: Uuid,
    pub guild_b_name: String,
    pub war_type: GuildWarType,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyEvent {
    pub party_id: Uuid,
    pub leader_id: CharacterId,
    pub member_ids: Vec<CharacterId>,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseEvent {
    pub house_id: u32,
    pub house_name: String,
    pub owner_id: Option<CharacterId>,
    pub previous_owner_id: Option<CharacterId>,
    pub price: u64,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementEvent {
    pub character_id: CharacterId,
    pub character_name: String,
    pub achievement_id: u32,
    pub achievement_name: String,
    pub points: u32,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestiaryEvent {
    pub character_id: CharacterId,
    pub character_name: String,
    pub creature_id: u32,
    pub creature_name: String,
    pub kills_required: u32,
    pub charm_points: u32,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionEvent {
    pub admin_id: PlayerId,
    pub admin_name: String,
    pub action: AdminAction,
    pub target: Option<String>,
    pub reason: Option<String>,
    pub realm_id: Option<RealmId>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceEvent {
    pub maintenance_type: MaintenanceType,
    pub scheduled_start: DateTime<Utc>,
    pub estimated_duration_minutes: u32,
    pub message: String,
    pub affected_realms: Vec<RealmId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmStatusEvent {
    pub realm_id: RealmId,
    pub realm_name: String,
    pub old_status: RealmStatus,
    pub new_status: RealmStatus,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchmakingEvent {
    pub match_id: Uuid,
    pub match_type: MatchType,
    pub participants: Vec<CharacterId>,
    pub realm_id: RealmId,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentEvent {
    pub tournament_id: Uuid,
    pub tournament_name: String,
    pub tournament_type: TournamentType,
    pub participants: Vec<CharacterId>,
    pub prize_pool: u64,
    pub realm_id: Option<RealmId>,
    pub timestamp: DateTime<Utc>,
}

// Supporting types

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootItem {
    pub item_id: u32,
    pub item_name: String,
    pub count: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Skill {
    Fist,
    Club,
    Sword,
    Axe,
    Distance,
    Shielding,
    Fishing,
    MagicLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Fire,
    Ice,
    Earth,
    Energy,
    Holy,
    Death,
    Drown,
    Healing,
    ManaDrain,
    LifeDrain,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SkullType {
    None,
    Yellow,
    Green,
    White,
    Red,
    Black,
    Orange,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ServerMessageType {
    Info,
    Warning,
    Error,
    Event,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BroadcastType {
    ServerWide,
    RealmWide,
    Channel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SeasonalEventType {
    Christmas,
    Halloween,
    Easter,
    Summer,
    Anniversary,
    Custom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GuildWarType {
    Declaration,
    Start,
    End,
    Surrender,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdminAction {
    Ban,
    Unban,
    Mute,
    Unmute,
    Kick,
    Teleport,
    CreateItem,
    DeleteItem,
    ModifyCharacter,
    ServerShutdown,
    ServerRestart,
    RealmCreate,
    RealmDelete,
    RealmModify,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MaintenanceType {
    Scheduled,
    Emergency,
    HotFix,
    Update,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RealmStatus {
    Online,
    Offline,
    Maintenance,
    Starting,
    Stopping,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MatchType {
    Duel,
    TeamDeathMatch,
    CaptureTheFlag,
    BattleRoyale,
    Ranked1v1,
    Ranked2v2,
    Ranked5v5,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TournamentType {
    SingleElimination,
    DoubleElimination,
    RoundRobin,
    Swiss,
    Custom,
}
