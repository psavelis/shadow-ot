//! Script Actions
//!
//! Defines actions that scripts can trigger in the game engine.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Context available to script execution
#[derive(Debug, Clone)]
pub struct ActionContext {
    /// Player who triggered the action
    pub player_id: Option<Uuid>,
    /// NPC involved (if any)
    pub npc_id: Option<Uuid>,
    /// Creature involved (if any)
    pub creature_id: Option<u32>,
    /// Item involved (if any)
    pub item_id: Option<u16>,
    /// Position of action
    pub position: Option<(u16, u16, u8)>,
    /// Custom variables
    pub variables: HashMap<String, String>,
}

impl ActionContext {
    pub fn new() -> Self {
        Self {
            player_id: None,
            npc_id: None,
            creature_id: None,
            item_id: None,
            position: None,
            variables: HashMap::new(),
        }
    }

    pub fn with_player(mut self, id: Uuid) -> Self {
        self.player_id = Some(id);
        self
    }

    pub fn with_npc(mut self, id: Uuid) -> Self {
        self.npc_id = Some(id);
        self
    }

    pub fn with_position(mut self, x: u16, y: u16, z: u8) -> Self {
        self.position = Some((x, y, z));
        self
    }

    pub fn with_var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(key.into(), value.into());
        self
    }
}

impl Default for ActionContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Actions that can be triggered by scripts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptAction {
    // Player Actions
    SendMessage {
        player_id: Uuid,
        message: String,
        message_type: MessageType,
    },
    Teleport {
        player_id: Uuid,
        x: u16,
        y: u16,
        z: u8,
    },
    GiveItem {
        player_id: Uuid,
        item_id: u16,
        count: u16,
    },
    RemoveItem {
        player_id: Uuid,
        item_id: u16,
        count: u16,
    },
    GiveExperience {
        player_id: Uuid,
        amount: u64,
    },
    Heal {
        player_id: Uuid,
        health: i32,
        mana: i32,
    },
    AddCondition {
        player_id: Uuid,
        condition: String,
        duration: u32,
    },
    RemoveCondition {
        player_id: Uuid,
        condition: String,
    },
    SetOutfit {
        player_id: Uuid,
        look_type: u16,
        head: u8,
        body: u8,
        legs: u8,
        feet: u8,
    },
    LearnSpell {
        player_id: Uuid,
        spell: String,
    },
    SetStorage {
        player_id: Uuid,
        key: u32,
        value: i32,
    },
    
    // Creature Actions
    SpawnCreature {
        name: String,
        x: u16,
        y: u16,
        z: u8,
    },
    RemoveCreature {
        creature_id: u32,
    },
    MoveCreature {
        creature_id: u32,
        x: u16,
        y: u16,
        z: u8,
    },
    CreatureSay {
        creature_id: u32,
        message: String,
        message_type: MessageType,
    },
    
    // World Actions
    CreateItem {
        item_id: u16,
        count: u16,
        x: u16,
        y: u16,
        z: u8,
    },
    RemoveItemAt {
        x: u16,
        y: u16,
        z: u8,
        item_id: u16,
    },
    TransformItem {
        from_id: u16,
        to_id: u16,
        x: u16,
        y: u16,
        z: u8,
    },
    PlayEffect {
        effect_id: u8,
        x: u16,
        y: u16,
        z: u8,
    },
    PlaySound {
        sound_id: u8,
        x: u16,
        y: u16,
        z: u8,
    },
    
    // Quest Actions
    StartQuest {
        player_id: Uuid,
        quest_id: String,
    },
    CompleteQuest {
        player_id: Uuid,
        quest_id: String,
    },
    UpdateQuest {
        player_id: Uuid,
        quest_id: String,
        value: i32,
    },
    
    // Shop Actions
    OpenShop {
        player_id: Uuid,
        shop_id: String,
    },
    
    // Dialog Actions
    OpenDialog {
        player_id: Uuid,
        npc_id: Uuid,
    },
    CloseDialog {
        player_id: Uuid,
    },
    
    // Custom
    Custom {
        name: String,
        params: HashMap<String, String>,
    },
}

/// Message types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Say,
    Whisper,
    Yell,
    Private,
    Channel,
    Broadcast,
    System,
    Info,
    Warning,
    Error,
    Loot,
    Experience,
    Damage,
    Heal,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Say
    }
}

/// Result of executing an action
#[derive(Debug, Clone)]
pub enum ActionResult {
    Success,
    Failed(String),
    Pending, // Action will complete asynchronously
}

/// Queue of pending actions to be executed
#[derive(Debug, Default)]
pub struct ActionQueue {
    actions: Vec<(ScriptAction, ActionContext)>,
}

impl ActionQueue {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Queue an action
    pub fn push(&mut self, action: ScriptAction, context: ActionContext) {
        self.actions.push((action, context));
    }

    /// Get and remove the next action
    pub fn pop(&mut self) -> Option<(ScriptAction, ActionContext)> {
        if self.actions.is_empty() {
            None
        } else {
            Some(self.actions.remove(0))
        }
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Number of pending actions
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Clear all actions
    pub fn clear(&mut self) {
        self.actions.clear();
    }

    /// Drain all actions
    pub fn drain(&mut self) -> Vec<(ScriptAction, ActionContext)> {
        std::mem::take(&mut self.actions)
    }
}

/// Trait for handling script actions
pub trait ActionHandler: Send + Sync {
    /// Execute an action
    fn execute(&self, action: &ScriptAction, context: &ActionContext) -> ActionResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_context() {
        let ctx = ActionContext::new()
            .with_player(Uuid::new_v4())
            .with_position(100, 100, 7)
            .with_var("test", "value");

        assert!(ctx.player_id.is_some());
        assert_eq!(ctx.position, Some((100, 100, 7)));
        assert_eq!(ctx.variables.get("test"), Some(&"value".to_string()));
    }

    #[test]
    fn test_action_queue() {
        let mut queue = ActionQueue::new();
        
        queue.push(
            ScriptAction::SendMessage {
                player_id: Uuid::new_v4(),
                message: "Test".to_string(),
                message_type: MessageType::Say,
            },
            ActionContext::new(),
        );

        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());

        let action = queue.pop();
        assert!(action.is_some());
        assert!(queue.is_empty());
    }
}
