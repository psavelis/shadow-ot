//! Party System
//!
//! Handles party management, experience sharing, and party coordination.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Party member status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyMemberStatus {
    /// Active party member
    Active,
    /// Invited but not yet joined
    Invited,
    /// Left the party but still tracked for shared exp
    Left,
}

/// Experience share mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharedExpMode {
    /// Experience shared equally
    Equal,
    /// Experience based on damage contribution
    Contribution,
    /// No experience sharing
    None,
}

impl Default for SharedExpMode {
    fn default() -> Self {
        Self::Equal
    }
}

/// Loot distribution mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyLootMode {
    /// Leader picks up all loot
    Leader,
    /// Random member for each drop
    Random,
    /// Round robin
    RoundRobin,
    /// Anyone can loot
    FreeForAll,
}

impl Default for PartyLootMode {
    fn default() -> Self {
        Self::Leader
    }
}

/// A party member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyMember {
    /// Player ID
    pub player_id: Uuid,
    /// Player name
    pub name: String,
    /// Player level
    pub level: u16,
    /// Player vocation
    pub vocation: String,
    /// Current health
    pub health: i32,
    /// Maximum health
    pub max_health: i32,
    /// Current mana
    pub mana: i32,
    /// Maximum mana
    pub max_mana: i32,
    /// Member status
    pub status: PartyMemberStatus,
    /// When joined
    pub joined_at: DateTime<Utc>,
    /// Total damage dealt (for contribution exp)
    pub damage_dealt: u64,
    /// Total healing done
    pub healing_done: u64,
    /// Shared exp active
    pub shared_exp_active: bool,
}

impl PartyMember {
    pub fn new(player_id: Uuid, name: impl Into<String>, level: u16) -> Self {
        Self {
            player_id,
            name: name.into(),
            level,
            vocation: String::new(),
            health: 100,
            max_health: 100,
            mana: 100,
            max_mana: 100,
            status: PartyMemberStatus::Active,
            joined_at: Utc::now(),
            damage_dealt: 0,
            healing_done: 0,
            shared_exp_active: true,
        }
    }

    pub fn invited(player_id: Uuid, name: impl Into<String>, level: u16) -> Self {
        let mut member = Self::new(player_id, name, level);
        member.status = PartyMemberStatus::Invited;
        member
    }

    pub fn is_active(&self) -> bool {
        self.status == PartyMemberStatus::Active
    }

    pub fn health_percent(&self) -> f32 {
        if self.max_health == 0 {
            return 0.0;
        }
        (self.health as f32 / self.max_health as f32) * 100.0
    }

    pub fn mana_percent(&self) -> f32 {
        if self.max_mana == 0 {
            return 0.0;
        }
        (self.mana as f32 / self.max_mana as f32) * 100.0
    }
}

/// Party invitation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyInvite {
    pub party_id: Uuid,
    pub inviter_id: Uuid,
    pub invitee_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl PartyInvite {
    pub fn new(party_id: Uuid, inviter_id: Uuid, invitee_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            party_id,
            inviter_id,
            invitee_id,
            created_at: now,
            expires_at: now + chrono::Duration::minutes(5),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

/// A party of players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    /// Party ID
    pub id: Uuid,
    /// Leader player ID
    pub leader_id: Uuid,
    /// Party members
    pub members: HashMap<Uuid, PartyMember>,
    /// Pending invitations
    pub invites: Vec<PartyInvite>,
    /// Experience share mode
    pub exp_mode: SharedExpMode,
    /// Loot distribution mode
    pub loot_mode: PartyLootMode,
    /// Round robin index for loot
    pub loot_robin_idx: usize,
    /// Total experience gained
    pub total_exp: u64,
    /// Party created at
    pub created_at: DateTime<Utc>,
    /// Minimum level for shared exp
    pub min_level: u16,
    /// Maximum level for shared exp
    pub max_level: u16,
}

impl Party {
    /// Create a new party
    pub fn new(leader_id: Uuid, leader_name: impl Into<String>, leader_level: u16) -> Self {
        let mut party = Self {
            id: Uuid::new_v4(),
            leader_id,
            members: HashMap::new(),
            invites: Vec::new(),
            exp_mode: SharedExpMode::Equal,
            loot_mode: PartyLootMode::Leader,
            loot_robin_idx: 0,
            total_exp: 0,
            created_at: Utc::now(),
            min_level: leader_level,
            max_level: leader_level,
        };

        party.members.insert(leader_id, PartyMember::new(leader_id, leader_name, leader_level));
        party
    }

    /// Get member count (active only)
    pub fn member_count(&self) -> usize {
        self.members.values().filter(|m| m.is_active()).count()
    }

    /// Get all active member IDs
    pub fn active_member_ids(&self) -> Vec<Uuid> {
        self.members
            .iter()
            .filter(|(_, m)| m.is_active())
            .map(|(id, _)| *id)
            .collect()
    }

    /// Check if player is leader
    pub fn is_leader(&self, player_id: Uuid) -> bool {
        self.leader_id == player_id
    }

    /// Check if player is member
    pub fn is_member(&self, player_id: Uuid) -> bool {
        self.members.get(&player_id)
            .map(|m| m.is_active())
            .unwrap_or(false)
    }

    /// Get member
    pub fn get_member(&self, player_id: Uuid) -> Option<&PartyMember> {
        self.members.get(&player_id)
    }

    /// Get member (mutable)
    pub fn get_member_mut(&mut self, player_id: Uuid) -> Option<&mut PartyMember> {
        self.members.get_mut(&player_id)
    }

    /// Invite a player
    pub fn invite(&mut self, invitee_id: Uuid, invitee_name: impl Into<String>, level: u16) -> Result<(), PartyError> {
        if self.is_member(invitee_id) {
            return Err(PartyError::AlreadyMember);
        }

        // Remove expired invites
        self.cleanup_invites();

        // Check if already invited
        if self.invites.iter().any(|i| i.invitee_id == invitee_id) {
            return Err(PartyError::AlreadyInvited);
        }

        // Add pending member
        self.members.insert(invitee_id, PartyMember::invited(invitee_id, invitee_name, level));
        self.invites.push(PartyInvite::new(self.id, self.leader_id, invitee_id));

        Ok(())
    }

    /// Accept invitation
    pub fn accept_invite(&mut self, player_id: Uuid) -> Result<(), PartyError> {
        // Check invite exists
        if !self.invites.iter().any(|i| i.invitee_id == player_id && !i.is_expired()) {
            return Err(PartyError::NotInvited);
        }

        // Remove invite
        self.invites.retain(|i| i.invitee_id != player_id);

        // Activate member
        if let Some(member) = self.members.get_mut(&player_id) {
            member.status = PartyMemberStatus::Active;
            member.joined_at = Utc::now();
            self.update_level_range();
        }

        Ok(())
    }

    /// Reject invitation
    pub fn reject_invite(&mut self, player_id: Uuid) {
        self.invites.retain(|i| i.invitee_id != player_id);
        self.members.remove(&player_id);
    }

    /// Remove member from party
    pub fn remove_member(&mut self, player_id: Uuid) -> Result<(), PartyError> {
        if self.is_leader(player_id) {
            return Err(PartyError::CannotKickLeader);
        }

        if !self.is_member(player_id) {
            return Err(PartyError::NotMember);
        }

        self.members.remove(&player_id);
        self.update_level_range();

        Ok(())
    }

    /// Leave party
    pub fn leave(&mut self, player_id: Uuid) -> Result<(), PartyError> {
        if self.is_leader(player_id) {
            // Transfer leadership or disband
            let active: Vec<_> = self.active_member_ids();
            if active.len() <= 1 {
                return Err(PartyError::PartyDisbanded);
            }
            // Transfer to next member
            if let Some(&new_leader) = active.iter().find(|&&id| id != player_id) {
                self.leader_id = new_leader;
            }
        }

        self.members.remove(&player_id);
        self.update_level_range();

        Ok(())
    }

    /// Pass leadership to another member
    pub fn pass_leadership(&mut self, new_leader_id: Uuid) -> Result<(), PartyError> {
        if !self.is_member(new_leader_id) {
            return Err(PartyError::NotMember);
        }

        self.leader_id = new_leader_id;
        Ok(())
    }

    /// Update level range for shared exp calculation
    fn update_level_range(&mut self) {
        let levels: Vec<u16> = self.members.values()
            .filter(|m| m.is_active())
            .map(|m| m.level)
            .collect();

        if let (Some(&min), Some(&max)) = (levels.iter().min(), levels.iter().max()) {
            self.min_level = min;
            self.max_level = max;
        }
    }

    /// Check if shared exp is possible (level range check)
    pub fn can_share_exp(&self) -> bool {
        // Max level difference is 2/3 of lowest member level
        let max_diff = (self.min_level as f32 * 2.0 / 3.0).ceil() as u16;
        (self.max_level - self.min_level) <= max_diff
    }

    /// Calculate shared exp for a kill
    pub fn calculate_shared_exp(&self, base_exp: u64, killer_id: Uuid) -> HashMap<Uuid, u64> {
        let mut distribution = HashMap::new();

        if !self.can_share_exp() || self.exp_mode == SharedExpMode::None {
            // Only killer gets exp
            distribution.insert(killer_id, base_exp);
            return distribution;
        }

        let active_members: Vec<_> = self.members.values()
            .filter(|m| m.is_active() && m.shared_exp_active)
            .collect();

        if active_members.is_empty() {
            return distribution;
        }

        // Bonus for party size (up to 50% bonus)
        let party_bonus = match active_members.len() {
            1 => 1.0,
            2 => 1.2,
            3 => 1.3,
            4 => 1.4,
            _ => 1.5,
        };

        let total_exp = (base_exp as f64 * party_bonus) as u64;

        match self.exp_mode {
            SharedExpMode::Equal => {
                let share = total_exp / active_members.len() as u64;
                for member in active_members {
                    distribution.insert(member.player_id, share);
                }
            }
            SharedExpMode::Contribution => {
                let total_damage: u64 = active_members.iter().map(|m| m.damage_dealt).sum();
                if total_damage == 0 {
                    // Fall back to equal distribution
                    let share = total_exp / active_members.len() as u64;
                    for member in active_members {
                        distribution.insert(member.player_id, share);
                    }
                } else {
                    for member in active_members {
                        let ratio = member.damage_dealt as f64 / total_damage as f64;
                        distribution.insert(member.player_id, (total_exp as f64 * ratio) as u64);
                    }
                }
            }
            SharedExpMode::None => {
                distribution.insert(killer_id, base_exp);
            }
        }

        distribution
    }

    /// Get next loot recipient
    pub fn get_loot_recipient(&mut self) -> Option<Uuid> {
        let active = self.active_member_ids();
        if active.is_empty() {
            return None;
        }

        match self.loot_mode {
            PartyLootMode::Leader => Some(self.leader_id),
            PartyLootMode::Random => {
                use rand::Rng;
                let idx = rand::thread_rng().gen_range(0..active.len());
                Some(active[idx])
            }
            PartyLootMode::RoundRobin => {
                let recipient = active[self.loot_robin_idx % active.len()];
                self.loot_robin_idx += 1;
                Some(recipient)
            }
            PartyLootMode::FreeForAll => None,
        }
    }

    /// Update member stats
    pub fn update_member_stats(&mut self, player_id: Uuid, health: i32, max_health: i32, mana: i32, max_mana: i32) {
        if let Some(member) = self.members.get_mut(&player_id) {
            member.health = health;
            member.max_health = max_health;
            member.mana = mana;
            member.max_mana = max_mana;
        }
    }

    /// Record damage dealt by member
    pub fn record_damage(&mut self, player_id: Uuid, damage: u64) {
        if let Some(member) = self.members.get_mut(&player_id) {
            member.damage_dealt += damage;
        }
    }

    /// Record healing done by member
    pub fn record_healing(&mut self, player_id: Uuid, healing: u64) {
        if let Some(member) = self.members.get_mut(&player_id) {
            member.healing_done += healing;
        }
    }

    /// Cleanup expired invites
    pub fn cleanup_invites(&mut self) {
        let expired: Vec<_> = self.invites.iter()
            .filter(|i| i.is_expired())
            .map(|i| i.invitee_id)
            .collect();

        for id in expired {
            self.members.remove(&id);
        }
        self.invites.retain(|i| !i.is_expired());
    }
}

/// Party manager
pub struct PartyManager {
    /// All parties
    parties: HashMap<Uuid, Arc<RwLock<Party>>>,
    /// Player -> Party mapping
    player_parties: HashMap<Uuid, Uuid>,
}

impl PartyManager {
    pub fn new() -> Self {
        Self {
            parties: HashMap::new(),
            player_parties: HashMap::new(),
        }
    }

    /// Create a new party
    pub fn create_party(&mut self, leader_id: Uuid, leader_name: impl Into<String>, level: u16) -> Result<Uuid, PartyError> {
        if self.player_parties.contains_key(&leader_id) {
            return Err(PartyError::AlreadyInParty);
        }

        let party = Party::new(leader_id, leader_name, level);
        let id = party.id;

        self.player_parties.insert(leader_id, id);
        self.parties.insert(id, Arc::new(RwLock::new(party)));

        Ok(id)
    }

    /// Get party by ID
    pub fn get(&self, id: Uuid) -> Option<Arc<RwLock<Party>>> {
        self.parties.get(&id).cloned()
    }

    /// Get party by player ID
    pub fn get_by_player(&self, player_id: Uuid) -> Option<Arc<RwLock<Party>>> {
        self.player_parties.get(&player_id)
            .and_then(|id| self.parties.get(id).cloned())
    }

    /// Check if player is in a party
    pub fn is_in_party(&self, player_id: Uuid) -> bool {
        self.player_parties.contains_key(&player_id)
    }

    /// Add player to party mapping
    pub fn add_player_mapping(&mut self, player_id: Uuid, party_id: Uuid) {
        self.player_parties.insert(player_id, party_id);
    }

    /// Remove player from party mapping
    pub fn remove_player_mapping(&mut self, player_id: Uuid) {
        self.player_parties.remove(&player_id);
    }

    /// Disband a party
    pub async fn disband(&mut self, party_id: Uuid) {
        if let Some(party) = self.parties.remove(&party_id) {
            let p = party.read().await;
            for member_id in p.members.keys() {
                self.player_parties.remove(member_id);
            }
        }
    }

    /// Get all parties
    pub fn all(&self) -> &HashMap<Uuid, Arc<RwLock<Party>>> {
        &self.parties
    }

    /// Party count
    pub fn count(&self) -> usize {
        self.parties.len()
    }
}

impl Default for PartyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Party errors
#[derive(Debug, Clone)]
pub enum PartyError {
    AlreadyInParty,
    AlreadyMember,
    AlreadyInvited,
    NotInvited,
    NotMember,
    NotLeader,
    CannotKickLeader,
    PartyDisbanded,
    PartyFull,
}

impl std::fmt::Display for PartyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartyError::AlreadyInParty => write!(f, "You are already in a party"),
            PartyError::AlreadyMember => write!(f, "Player is already a party member"),
            PartyError::AlreadyInvited => write!(f, "Player is already invited"),
            PartyError::NotInvited => write!(f, "Player was not invited"),
            PartyError::NotMember => write!(f, "Player is not a party member"),
            PartyError::NotLeader => write!(f, "Only the party leader can do this"),
            PartyError::CannotKickLeader => write!(f, "Cannot kick the party leader"),
            PartyError::PartyDisbanded => write!(f, "The party has been disbanded"),
            PartyError::PartyFull => write!(f, "The party is full"),
        }
    }
}

impl std::error::Error for PartyError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_party_creation() {
        let leader_id = Uuid::new_v4();
        let party = Party::new(leader_id, "Leader", 100);

        assert!(party.is_leader(leader_id));
        assert!(party.is_member(leader_id));
        assert_eq!(party.member_count(), 1);
    }

    #[test]
    fn test_party_invite() {
        let leader_id = Uuid::new_v4();
        let player_id = Uuid::new_v4();
        let mut party = Party::new(leader_id, "Leader", 100);

        party.invite(player_id, "Player", 95).unwrap();
        assert!(!party.is_member(player_id)); // Still invited

        party.accept_invite(player_id).unwrap();
        assert!(party.is_member(player_id));
        assert_eq!(party.member_count(), 2);
    }

    #[test]
    fn test_shared_exp() {
        let leader_id = Uuid::new_v4();
        let player_id = Uuid::new_v4();
        let mut party = Party::new(leader_id, "Leader", 100);

        party.invite(player_id, "Player", 95).unwrap();
        party.accept_invite(player_id).unwrap();

        let exp = party.calculate_shared_exp(1000, leader_id);
        assert_eq!(exp.len(), 2);
        
        // Should have party bonus
        let total: u64 = exp.values().sum();
        assert!(total > 1000); // 20% bonus for 2 members
    }

    #[test]
    fn test_level_range() {
        let leader_id = Uuid::new_v4();
        let party = Party::new(leader_id, "Leader", 100);

        assert!(party.can_share_exp());
    }

    #[test]
    fn test_party_manager() {
        let mut manager = PartyManager::new();
        let leader_id = Uuid::new_v4();

        let party_id = manager.create_party(leader_id, "Leader", 100).unwrap();
        assert!(manager.get(party_id).is_some());
        assert!(manager.is_in_party(leader_id));
    }
}
