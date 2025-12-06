//! Realm Manager
//!
//! Manages multiple realm instances.

use std::collections::HashMap;
use uuid::Uuid;

use crate::{RealmConfig, RealmError, RealmInfo, RealmInstance, RealmListResponse, RealmStatus, RealmType};

/// Manages all realm instances
pub struct RealmManager {
    /// Active realm instances
    realms: HashMap<Uuid, RealmInstance>,
    /// Default realm ID
    default_realm: Option<Uuid>,
    /// Featured realms
    featured: Vec<Uuid>,
}

impl RealmManager {
    /// Create a new realm manager
    pub fn new() -> Self {
        Self {
            realms: HashMap::new(),
            default_realm: None,
            featured: Vec::new(),
        }
    }

    /// Create a new realm
    pub fn create_realm(
        &mut self,
        name: &str,
        config: RealmConfig,
    ) -> Result<Uuid, RealmError> {
        let instance = RealmInstance::new(name, config);
        let id = instance.info.id;
        
        self.realms.insert(id, instance);
        
        // Set as default if first realm
        if self.default_realm.is_none() {
            self.default_realm = Some(id);
        }
        
        Ok(id)
    }

    /// Get a realm instance
    pub fn get_realm(&self, realm_id: Uuid) -> Option<&RealmInstance> {
        self.realms.get(&realm_id)
    }

    /// Get mutable realm instance
    pub fn get_realm_mut(&mut self, realm_id: Uuid) -> Option<&mut RealmInstance> {
        self.realms.get_mut(&realm_id)
    }

    /// Start a realm
    pub fn start_realm(&mut self, realm_id: Uuid) -> Result<(), RealmError> {
        let realm = self.realms.get_mut(&realm_id)
            .ok_or(RealmError::NotFound(realm_id))?;
        realm.start()
    }

    /// Stop a realm
    pub fn stop_realm(&mut self, realm_id: Uuid) -> Result<(), RealmError> {
        let realm = self.realms.get_mut(&realm_id)
            .ok_or(RealmError::NotFound(realm_id))?;
        realm.stop();
        Ok(())
    }

    /// Get realm list for a player
    pub fn get_realm_list(&self, account_id: Uuid) -> RealmListResponse {
        let realms: Vec<RealmInfo> = self.realms.values()
            .map(|r| r.info.clone())
            .collect();

        let player_realms: Vec<Uuid> = self.realms.values()
            .filter(|r| r.online_players.values().any(|&a| a == account_id))
            .map(|r| r.info.id)
            .collect();

        let recommended = self.get_recommended_realm();

        RealmListResponse {
            realms,
            recommended,
            player_realms,
            last_realm: None, // Would need to be looked up from database
        }
    }

    /// Get recommended realm for new players
    fn get_recommended_realm(&self) -> Option<Uuid> {
        // Prefer featured realms that aren't full
        for &id in &self.featured {
            if let Some(realm) = self.realms.get(&id) {
                if realm.info.is_available() {
                    return Some(id);
                }
            }
        }

        // Fall back to default
        if let Some(default_id) = self.default_realm {
            if let Some(realm) = self.realms.get(&default_id) {
                if realm.info.is_available() {
                    return Some(default_id);
                }
            }
        }

        // Find any available realm
        self.realms.values()
            .filter(|r| r.info.is_available())
            .min_by_key(|r| r.online_count())
            .map(|r| r.info.id)
    }

    /// Get all online realms
    pub fn get_online_realms(&self) -> Vec<&RealmInstance> {
        self.realms.values()
            .filter(|r| matches!(r.info.status, RealmStatus::Online))
            .collect()
    }

    /// Get realms by type
    pub fn get_realms_by_type(&self, realm_type: RealmType) -> Vec<&RealmInstance> {
        self.realms.values()
            .filter(|r| r.info.realm_type == realm_type)
            .collect()
    }

    /// Get total online players across all realms
    pub fn total_online(&self) -> usize {
        self.realms.values()
            .map(|r| r.online_count())
            .sum()
    }

    /// Find which realm a character is on
    pub fn find_character_realm(&self, character_id: Uuid) -> Option<Uuid> {
        self.realms.values()
            .find(|r| r.is_online(character_id))
            .map(|r| r.info.id)
    }

    /// Set default realm
    pub fn set_default_realm(&mut self, realm_id: Uuid) -> Result<(), RealmError> {
        if self.realms.contains_key(&realm_id) {
            self.default_realm = Some(realm_id);
            Ok(())
        } else {
            Err(RealmError::NotFound(realm_id))
        }
    }

    /// Set featured realms
    pub fn set_featured(&mut self, realm_ids: Vec<Uuid>) {
        self.featured = realm_ids.into_iter()
            .filter(|id| self.realms.contains_key(id))
            .collect();
    }

    /// Remove a realm
    pub fn remove_realm(&mut self, realm_id: Uuid) -> Result<(), RealmError> {
        // Can't remove if players online
        if let Some(realm) = self.realms.get(&realm_id) {
            if realm.online_count() > 0 {
                return Err(RealmError::ConfigError("Cannot remove realm with players online".to_string()));
            }
        }

        self.realms.remove(&realm_id)
            .map(|_| ())
            .ok_or(RealmError::NotFound(realm_id))
    }

    /// Process all realms (save, cleanup, etc.)
    pub fn process(&mut self) {
        for realm in self.realms.values_mut() {
            // Check for idle sessions
            // Auto-save if needed
            // Other maintenance tasks
        }
    }
}

impl Default for RealmManager {
    fn default() -> Self {
        Self::new()
    }
}
