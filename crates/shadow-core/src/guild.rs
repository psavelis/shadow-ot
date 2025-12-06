//! Guild System
//!
//! Handles guild management, ranks, members, wars, and guild halls.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Guild rank permissions (bitmask)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuildPermissions(u32);

impl GuildPermissions {
    pub const NONE: u32 = 0;
    pub const INVITE: u32 = 1 << 0;
    pub const KICK: u32 = 1 << 1;
    pub const EDIT_MOTD: u32 = 1 << 2;
    pub const EDIT_RANKS: u32 = 1 << 3;
    pub const WAR_DECLARE: u32 = 1 << 4;
    pub const WAR_ACCEPT: u32 = 1 << 5;
    pub const BANK_DEPOSIT: u32 = 1 << 6;
    pub const BANK_WITHDRAW: u32 = 1 << 7;
    pub const MANAGE_HALL: u32 = 1 << 8;
    pub const PROMOTE: u32 = 1 << 9;
    pub const DISBAND: u32 = 1 << 10;

    pub const LEADER: u32 = 0xFFFFFFFF; // All permissions
    pub const VICE_LEADER: u32 = Self::INVITE | Self::KICK | Self::EDIT_MOTD | Self::WAR_ACCEPT | Self::PROMOTE;
    pub const MEMBER: u32 = Self::BANK_DEPOSIT;

    pub fn new(bits: u32) -> Self {
        Self(bits)
    }

    pub fn has(&self, permission: u32) -> bool {
        (self.0 & permission) != 0
    }

    pub fn grant(&mut self, permission: u32) {
        self.0 |= permission;
    }

    pub fn revoke(&mut self, permission: u32) {
        self.0 &= !permission;
    }
}

impl Default for GuildPermissions {
    fn default() -> Self {
        Self(Self::MEMBER)
    }
}

/// Guild rank
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRank {
    /// Rank ID
    pub id: u32,
    /// Rank name
    pub name: String,
    /// Rank level (1 = leader, higher = lower rank)
    pub level: u8,
    /// Permissions
    pub permissions: GuildPermissions,
}

impl GuildRank {
    pub fn new(id: u32, name: impl Into<String>, level: u8) -> Self {
        Self {
            id,
            name: name.into(),
            level,
            permissions: GuildPermissions::default(),
        }
    }

    pub fn leader(id: u32) -> Self {
        Self {
            id,
            name: "Leader".to_string(),
            level: 1,
            permissions: GuildPermissions::new(GuildPermissions::LEADER),
        }
    }

    pub fn vice_leader(id: u32) -> Self {
        Self {
            id,
            name: "Vice-Leader".to_string(),
            level: 2,
            permissions: GuildPermissions::new(GuildPermissions::VICE_LEADER),
        }
    }

    pub fn member(id: u32) -> Self {
        Self {
            id,
            name: "Member".to_string(),
            level: 3,
            permissions: GuildPermissions::new(GuildPermissions::MEMBER),
        }
    }
}

/// Guild member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMember {
    /// Player ID
    pub player_id: Uuid,
    /// Player name
    pub name: String,
    /// Rank ID
    pub rank_id: u32,
    /// Join date
    pub joined_at: DateTime<Utc>,
    /// Nick within guild
    pub nick: Option<String>,
    /// Online status
    pub online: bool,
    /// Level (cached)
    pub level: u16,
    /// Vocation (cached)
    pub vocation: String,
}

impl GuildMember {
    pub fn new(player_id: Uuid, name: impl Into<String>, rank_id: u32) -> Self {
        Self {
            player_id,
            name: name.into(),
            rank_id,
            joined_at: Utc::now(),
            nick: None,
            online: false,
            level: 0,
            vocation: String::new(),
        }
    }
}

/// Guild invite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildInvite {
    pub guild_id: Uuid,
    pub player_id: Uuid,
    pub inviter_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl GuildInvite {
    pub fn new(guild_id: Uuid, player_id: Uuid, inviter_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            guild_id,
            player_id,
            inviter_id,
            created_at: now,
            expires_at: now + chrono::Duration::days(7),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

/// Guild war status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarStatus {
    Pending,
    Active,
    Ended,
    Rejected,
}

/// Guild war
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildWar {
    /// War ID
    pub id: Uuid,
    /// Attacking guild
    pub attacker_id: Uuid,
    /// Defending guild
    pub defender_id: Uuid,
    /// Status
    pub status: WarStatus,
    /// Kill limit (0 = unlimited)
    pub kill_limit: u32,
    /// Duration in days (0 = unlimited)
    pub duration_days: u32,
    /// Frags by attacker
    pub attacker_kills: u32,
    /// Frags by defender
    pub defender_kills: u32,
    /// War declaration date
    pub declared_at: DateTime<Utc>,
    /// War start date
    pub started_at: Option<DateTime<Utc>>,
    /// War end date
    pub ended_at: Option<DateTime<Utc>>,
    /// Entry fee
    pub entry_fee: u64,
}

impl GuildWar {
    pub fn new(attacker_id: Uuid, defender_id: Uuid, kill_limit: u32, duration_days: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            attacker_id,
            defender_id,
            status: WarStatus::Pending,
            kill_limit,
            duration_days,
            attacker_kills: 0,
            defender_kills: 0,
            declared_at: Utc::now(),
            started_at: None,
            ended_at: None,
            entry_fee: 0,
        }
    }

    pub fn accept(&mut self) {
        self.status = WarStatus::Active;
        self.started_at = Some(Utc::now());
    }

    pub fn reject(&mut self) {
        self.status = WarStatus::Rejected;
        self.ended_at = Some(Utc::now());
    }

    pub fn end(&mut self) {
        self.status = WarStatus::Ended;
        self.ended_at = Some(Utc::now());
    }

    pub fn is_active(&self) -> bool {
        self.status == WarStatus::Active
    }

    pub fn winner(&self) -> Option<Uuid> {
        if self.status != WarStatus::Ended {
            return None;
        }
        if self.kill_limit > 0 {
            if self.attacker_kills >= self.kill_limit {
                return Some(self.attacker_id);
            }
            if self.defender_kills >= self.kill_limit {
                return Some(self.defender_id);
            }
        }
        if self.attacker_kills > self.defender_kills {
            Some(self.attacker_id)
        } else if self.defender_kills > self.attacker_kills {
            Some(self.defender_id)
        } else {
            None // Draw
        }
    }
}

/// A guild
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    /// Guild ID
    pub id: Uuid,
    /// Guild name
    pub name: String,
    /// Owner (leader) player ID
    pub owner_id: Uuid,
    /// Creation date
    pub created_at: DateTime<Utc>,
    /// Message of the day
    pub motd: String,
    /// Guild description
    pub description: String,
    /// Guild ranks
    pub ranks: Vec<GuildRank>,
    /// Guild members
    pub members: HashMap<Uuid, GuildMember>,
    /// Bank balance
    pub balance: u64,
    /// Guild hall house ID
    pub guild_hall_id: Option<u32>,
    /// Guild logo (sprite ID or custom)
    pub logo: Option<u16>,
    /// Active wars
    pub wars: Vec<GuildWar>,
    /// Pending invites
    pub invites: Vec<GuildInvite>,
}

impl Guild {
    /// Create a new guild
    pub fn new(name: impl Into<String>, owner_id: Uuid, owner_name: impl Into<String>) -> Self {
        let mut guild = Self {
            id: Uuid::new_v4(),
            name: name.into(),
            owner_id,
            created_at: Utc::now(),
            motd: String::new(),
            description: String::new(),
            ranks: Vec::new(),
            members: HashMap::new(),
            balance: 0,
            guild_hall_id: None,
            logo: None,
            wars: Vec::new(),
            invites: Vec::new(),
        };

        // Create default ranks
        guild.ranks.push(GuildRank::leader(1));
        guild.ranks.push(GuildRank::vice_leader(2));
        guild.ranks.push(GuildRank::member(3));

        // Add owner as leader
        guild.members.insert(owner_id, GuildMember {
            player_id: owner_id,
            name: owner_name.into(),
            rank_id: 1,
            joined_at: Utc::now(),
            nick: None,
            online: true,
            level: 0,
            vocation: String::new(),
        });

        guild
    }

    /// Get member count
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Get online member count
    pub fn online_count(&self) -> usize {
        self.members.values().filter(|m| m.online).count()
    }

    /// Get member by player ID
    pub fn get_member(&self, player_id: Uuid) -> Option<&GuildMember> {
        self.members.get(&player_id)
    }

    /// Get member by player ID (mutable)
    pub fn get_member_mut(&mut self, player_id: Uuid) -> Option<&mut GuildMember> {
        self.members.get_mut(&player_id)
    }

    /// Check if player is member
    pub fn is_member(&self, player_id: Uuid) -> bool {
        self.members.contains_key(&player_id)
    }

    /// Check if player is leader
    pub fn is_leader(&self, player_id: Uuid) -> bool {
        self.owner_id == player_id
    }

    /// Get rank by ID
    pub fn get_rank(&self, rank_id: u32) -> Option<&GuildRank> {
        self.ranks.iter().find(|r| r.id == rank_id)
    }

    /// Get member's rank
    pub fn get_member_rank(&self, player_id: Uuid) -> Option<&GuildRank> {
        self.members.get(&player_id)
            .and_then(|m| self.get_rank(m.rank_id))
    }

    /// Check if member has permission
    pub fn has_permission(&self, player_id: Uuid, permission: u32) -> bool {
        if self.is_leader(player_id) {
            return true;
        }
        self.get_member_rank(player_id)
            .map(|r| r.permissions.has(permission))
            .unwrap_or(false)
    }

    /// Add member
    pub fn add_member(&mut self, member: GuildMember) -> bool {
        if self.is_member(member.player_id) {
            return false;
        }
        self.members.insert(member.player_id, member);
        true
    }

    /// Remove member
    pub fn remove_member(&mut self, player_id: Uuid) -> bool {
        if self.is_leader(player_id) {
            return false; // Can't remove leader
        }
        self.members.remove(&player_id).is_some()
    }

    /// Promote member to rank
    pub fn promote(&mut self, player_id: Uuid, rank_id: u32) -> bool {
        if let Some(member) = self.members.get_mut(&player_id) {
            if self.ranks.iter().any(|r| r.id == rank_id) {
                member.rank_id = rank_id;
                return true;
            }
        }
        false
    }

    /// Create invite
    pub fn invite(&mut self, player_id: Uuid, inviter_id: Uuid) {
        // Remove existing invite for this player
        self.invites.retain(|i| i.player_id != player_id);
        self.invites.push(GuildInvite::new(self.id, player_id, inviter_id));
    }

    /// Check if player has pending invite
    pub fn has_invite(&self, player_id: Uuid) -> bool {
        self.invites.iter().any(|i| i.player_id == player_id && !i.is_expired())
    }

    /// Accept invite
    pub fn accept_invite(&mut self, player_id: Uuid, player_name: impl Into<String>) -> bool {
        if !self.has_invite(player_id) {
            return false;
        }

        // Remove invite
        self.invites.retain(|i| i.player_id != player_id);

        // Add as member with default rank
        let default_rank = self.ranks.iter().max_by_key(|r| r.level).map(|r| r.id).unwrap_or(3);
        self.add_member(GuildMember::new(player_id, player_name, default_rank))
    }

    /// Reject invite
    pub fn reject_invite(&mut self, player_id: Uuid) {
        self.invites.retain(|i| i.player_id != player_id);
    }

    /// Declare war
    pub fn declare_war(&mut self, target_guild_id: Uuid, kill_limit: u32, duration_days: u32) -> GuildWar {
        let war = GuildWar::new(self.id, target_guild_id, kill_limit, duration_days);
        self.wars.push(war.clone());
        war
    }

    /// Get active wars
    pub fn active_wars(&self) -> Vec<&GuildWar> {
        self.wars.iter().filter(|w| w.is_active()).collect()
    }

    /// Check if at war with guild
    pub fn is_at_war_with(&self, guild_id: Uuid) -> bool {
        self.wars.iter().any(|w| {
            w.is_active() && (w.attacker_id == guild_id || w.defender_id == guild_id)
        })
    }

    /// Clean up expired invites
    pub fn cleanup_invites(&mut self) {
        self.invites.retain(|i| !i.is_expired());
    }
}

/// Guild manager
pub struct GuildManager {
    guilds: HashMap<Uuid, Arc<RwLock<Guild>>>,
    /// Player ID -> Guild ID mapping
    player_guilds: HashMap<Uuid, Uuid>,
}

impl GuildManager {
    pub fn new() -> Self {
        Self {
            guilds: HashMap::new(),
            player_guilds: HashMap::new(),
        }
    }

    /// Create a new guild
    pub async fn create_guild(
        &mut self,
        name: impl Into<String>,
        owner_id: Uuid,
        owner_name: impl Into<String>,
    ) -> Result<Uuid, GuildError> {
        let name = name.into();

        // Check if player already in a guild
        if self.player_guilds.contains_key(&owner_id) {
            return Err(GuildError::AlreadyInGuild);
        }

        // Check if guild name is taken
        for guild in self.guilds.values() {
            let g = guild.read().await;
            if g.name.eq_ignore_ascii_case(&name) {
                return Err(GuildError::NameTaken);
            }
        }

        let guild = Guild::new(name, owner_id, owner_name);
        let id = guild.id;

        self.player_guilds.insert(owner_id, id);
        self.guilds.insert(id, Arc::new(RwLock::new(guild)));

        Ok(id)
    }

    /// Get guild by ID
    pub fn get(&self, id: Uuid) -> Option<Arc<RwLock<Guild>>> {
        self.guilds.get(&id).cloned()
    }

    /// Get guild by player ID
    pub fn get_by_player(&self, player_id: Uuid) -> Option<Arc<RwLock<Guild>>> {
        self.player_guilds.get(&player_id)
            .and_then(|id| self.guilds.get(id).cloned())
    }

    /// Disband guild
    pub async fn disband(&mut self, guild_id: Uuid, requester_id: Uuid) -> Result<(), GuildError> {
        let guild = self.guilds.get(&guild_id).ok_or(GuildError::NotFound)?;
        
        {
            let g = guild.read().await;
            if !g.is_leader(requester_id) {
                return Err(GuildError::NoPermission);
            }

            // Remove all player mappings
            for member_id in g.members.keys() {
                self.player_guilds.remove(member_id);
            }
        }

        self.guilds.remove(&guild_id);
        Ok(())
    }

    /// Add player to guild mapping
    pub fn add_player_mapping(&mut self, player_id: Uuid, guild_id: Uuid) {
        self.player_guilds.insert(player_id, guild_id);
    }

    /// Remove player from guild mapping
    pub fn remove_player_mapping(&mut self, player_id: Uuid) {
        self.player_guilds.remove(&player_id);
    }

    /// Get all guilds
    pub fn all(&self) -> &HashMap<Uuid, Arc<RwLock<Guild>>> {
        &self.guilds
    }

    /// Guild count
    pub fn count(&self) -> usize {
        self.guilds.len()
    }
}

impl Default for GuildManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Guild errors
#[derive(Debug, Clone)]
pub enum GuildError {
    NotFound,
    AlreadyInGuild,
    NameTaken,
    NoPermission,
    InvalidRank,
    NotMember,
    InsufficientFunds,
}

impl std::fmt::Display for GuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuildError::NotFound => write!(f, "Guild not found"),
            GuildError::AlreadyInGuild => write!(f, "Player is already in a guild"),
            GuildError::NameTaken => write!(f, "Guild name is already taken"),
            GuildError::NoPermission => write!(f, "No permission to perform this action"),
            GuildError::InvalidRank => write!(f, "Invalid rank"),
            GuildError::NotMember => write!(f, "Player is not a member"),
            GuildError::InsufficientFunds => write!(f, "Insufficient guild funds"),
        }
    }
}

impl std::error::Error for GuildError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guild_creation() {
        let owner_id = Uuid::new_v4();
        let guild = Guild::new("Test Guild", owner_id, "TestLeader");

        assert_eq!(guild.name, "Test Guild");
        assert!(guild.is_leader(owner_id));
        assert!(guild.is_member(owner_id));
        assert_eq!(guild.member_count(), 1);
    }

    #[test]
    fn test_guild_permissions() {
        let owner_id = Uuid::new_v4();
        let guild = Guild::new("Test", owner_id, "Leader");

        assert!(guild.has_permission(owner_id, GuildPermissions::DISBAND));
        assert!(guild.has_permission(owner_id, GuildPermissions::INVITE));
    }

    #[test]
    fn test_guild_invite() {
        let owner_id = Uuid::new_v4();
        let player_id = Uuid::new_v4();
        let mut guild = Guild::new("Test", owner_id, "Leader");

        guild.invite(player_id, owner_id);
        assert!(guild.has_invite(player_id));

        assert!(guild.accept_invite(player_id, "Player"));
        assert!(guild.is_member(player_id));
        assert_eq!(guild.member_count(), 2);
    }

    #[test]
    fn test_guild_war() {
        let owner_id = Uuid::new_v4();
        let mut guild = Guild::new("Test", owner_id, "Leader");
        let enemy_id = Uuid::new_v4();

        let mut war = guild.declare_war(enemy_id, 100, 7);
        assert_eq!(war.status, WarStatus::Pending);

        war.accept();
        assert!(war.is_active());
    }

    #[tokio::test]
    async fn test_guild_manager() {
        let mut manager = GuildManager::new();
        let owner_id = Uuid::new_v4();

        let guild_id = manager.create_guild("Test Guild", owner_id, "Leader").await.unwrap();
        
        assert!(manager.get(guild_id).is_some());
        assert!(manager.get_by_player(owner_id).is_some());

        // Can't create another guild
        let result = manager.create_guild("Another", owner_id, "Leader").await;
        assert!(matches!(result, Err(GuildError::AlreadyInGuild)));
    }
}
