//! Shadow OT Scripting Engine
//!
//! Provides scripting capabilities for NPCs, quests, items, and game events.
//! Supports Lua scripting with a comprehensive API.

pub mod npc;
pub mod dialog;
pub mod shop;
pub mod quest;
pub mod lua;
pub mod actions;

pub use npc::{Npc, NpcHandler, NpcManager};
pub use dialog::{DialogHandler, DialogState, DialogResponse};
pub use shop::{Shop, ShopItem, ShopHandler};
pub use quest::{QuestScript, QuestTrigger};
pub use lua::LuaEngine;
pub use actions::{ScriptAction, ActionContext};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ScriptError>;

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("Lua error: {0}")]
    Lua(String),

    #[error("Script not found: {0}")]
    NotFound(String),

    #[error("Invalid script: {0}")]
    Invalid(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("NPC not found: {0}")]
    NpcNotFound(String),

    #[error("Shop not found: {0}")]
    ShopNotFound(String),

    #[error("Quest error: {0}")]
    Quest(String),

    #[error("Dialog error: {0}")]
    Dialog(String),
}
