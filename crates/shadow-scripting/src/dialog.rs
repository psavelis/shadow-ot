//! Dialog System
//!
//! Handles NPC conversations with keyword matching and state management.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Result, ScriptError};

/// State of an active dialog
#[derive(Debug, Clone, Default)]
pub struct DialogState {
    /// Current dialog topic
    pub topic: Option<String>,
    /// Variables set during dialog
    pub variables: HashMap<String, String>,
    /// Stack of previous topics
    pub topic_stack: Vec<String>,
}

impl DialogState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set current topic
    pub fn set_topic(&mut self, topic: impl Into<String>) {
        if let Some(current) = self.topic.take() {
            self.topic_stack.push(current);
        }
        self.topic = Some(topic.into());
    }

    /// Go back to previous topic
    pub fn pop_topic(&mut self) {
        self.topic = self.topic_stack.pop();
    }

    /// Set a variable
    pub fn set_var(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }

    /// Get a variable
    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    /// Clear state
    pub fn clear(&mut self) {
        self.topic = None;
        self.variables.clear();
        self.topic_stack.clear();
    }
}

/// A single dialog response definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogResponse {
    /// Keywords that trigger this response (any match)
    pub keywords: Vec<String>,
    /// Response text
    pub text: String,
    /// Optional topic to switch to
    pub set_topic: Option<String>,
    /// Required topic to be in
    pub require_topic: Option<String>,
    /// Whether this response is only valid once
    pub once: bool,
    /// Variable conditions
    #[serde(default)]
    pub conditions: HashMap<String, String>,
    /// Variables to set
    #[serde(default)]
    pub set_vars: HashMap<String, String>,
    /// Action to trigger
    pub action: Option<DialogAction>,
}

impl DialogResponse {
    /// Create a simple response
    pub fn new(keywords: Vec<&str>, text: impl Into<String>) -> Self {
        Self {
            keywords: keywords.into_iter().map(String::from).collect(),
            text: text.into(),
            set_topic: None,
            require_topic: None,
            once: false,
            conditions: HashMap::new(),
            set_vars: HashMap::new(),
            action: None,
        }
    }

    /// Check if message matches this response
    pub fn matches(&self, message: &str, state: &DialogState) -> bool {
        // Check topic requirement
        if let Some(ref required_topic) = self.require_topic {
            if state.topic.as_ref() != Some(required_topic) {
                return false;
            }
        }

        // Check conditions
        for (key, value) in &self.conditions {
            if state.get_var(key) != Some(value) {
                return false;
            }
        }

        // Check keywords
        let message_lower = message.to_lowercase();
        self.keywords.iter().any(|kw| message_lower.contains(kw))
    }
}

/// Action triggered by dialog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogAction {
    /// Open shop window
    OpenShop(String),
    /// Give item to player
    GiveItem { item_id: u16, count: u16 },
    /// Take item from player
    TakeItem { item_id: u16, count: u16 },
    /// Start quest
    StartQuest(String),
    /// Complete quest
    CompleteQuest(String),
    /// Teleport player
    Teleport { x: u16, y: u16, z: u8 },
    /// Cast spell on player
    CastSpell(String),
    /// Heal player
    Heal { amount: u32 },
    /// Custom Lua callback
    Callback(String),
}

/// Handles dialog processing for NPCs
pub struct DialogHandler {
    /// All possible responses
    responses: Vec<DialogResponse>,
    /// Current dialog state
    state: DialogState,
    /// Greeting patterns
    greeting_patterns: Vec<Regex>,
    /// Farewell patterns  
    farewell_patterns: Vec<Regex>,
    /// Default greeting text
    default_greeting: String,
    /// Default farewell text
    default_farewell: String,
    /// Default "don't understand" text
    default_unknown: String,
    /// Used responses (for "once" responses)
    used_responses: Vec<usize>,
}

impl DialogHandler {
    /// Create a new dialog handler
    pub fn new() -> Self {
        Self {
            responses: Vec::new(),
            state: DialogState::new(),
            greeting_patterns: vec![
                Regex::new(r"(?i)\bhi\b").unwrap(),
                Regex::new(r"(?i)\bhello\b").unwrap(),
                Regex::new(r"(?i)\bhey\b").unwrap(),
                Regex::new(r"(?i)\bgreetings?\b").unwrap(),
            ],
            farewell_patterns: vec![
                Regex::new(r"(?i)\bbye\b").unwrap(),
                Regex::new(r"(?i)\bfarewells?\b").unwrap(),
                Regex::new(r"(?i)\bgoodbye\b").unwrap(),
                Regex::new(r"(?i)\bciao\b").unwrap(),
            ],
            default_greeting: "Hello, traveler! How may I help you?".to_string(),
            default_farewell: "Farewell!".to_string(),
            default_unknown: "I don't understand what you mean.".to_string(),
            used_responses: Vec::new(),
        }
    }

    /// Add a response
    pub fn add_response(&mut self, response: DialogResponse) {
        self.responses.push(response);
    }

    /// Load responses from JSON
    pub fn load_from_json(&mut self, json: &str) -> Result<()> {
        let responses: Vec<DialogResponse> = serde_json::from_str(json)
            .map_err(|e| ScriptError::Dialog(e.to_string()))?;
        self.responses.extend(responses);
        Ok(())
    }

    /// Check if message is a greeting
    pub fn is_greeting(&self, message: &str) -> bool {
        self.greeting_patterns.iter().any(|p| p.is_match(message))
    }

    /// Check if message is a farewell
    pub fn is_farewell(&self, message: &str) -> bool {
        self.farewell_patterns.iter().any(|p| p.is_match(message))
    }

    /// Get greeting response
    pub fn get_greeting(&self, npc_name: &str) -> Option<String> {
        Some(self.default_greeting.replace("{npc}", npc_name))
    }

    /// Get farewell response
    pub fn get_farewell(&self, npc_name: &str) -> Option<String> {
        self.state.clone(); // Clear on farewell
        Some(self.default_farewell.replace("{npc}", npc_name))
    }

    /// Process a message and return response
    pub fn process_message(&mut self, message: &str) -> Option<String> {
        // Find matching response
        for (idx, response) in self.responses.iter().enumerate() {
            // Skip if "once" and already used
            if response.once && self.used_responses.contains(&idx) {
                continue;
            }

            if response.matches(message, &self.state) {
                // Mark as used if "once"
                if response.once {
                    self.used_responses.push(idx);
                }

                // Update state
                if let Some(ref topic) = response.set_topic {
                    self.state.set_topic(topic);
                }
                for (key, value) in &response.set_vars {
                    self.state.set_var(key, value);
                }

                // Return response text
                return Some(self.expand_variables(&response.text));
            }
        }

        // No match found
        Some(self.default_unknown.clone())
    }

    /// Get pending action if any
    pub fn get_pending_action(&self) -> Option<&DialogAction> {
        // Find most recent response that matched and has an action
        for response in &self.responses {
            if response.action.is_some() && response.matches("", &self.state) {
                return response.action.as_ref();
            }
        }
        None
    }

    /// Expand variables in text
    fn expand_variables(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (key, value) in &self.state.variables {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    /// Set default greeting
    pub fn set_greeting(&mut self, text: impl Into<String>) {
        self.default_greeting = text.into();
    }

    /// Set default farewell
    pub fn set_farewell(&mut self, text: impl Into<String>) {
        self.default_farewell = text.into();
    }

    /// Set unknown response
    pub fn set_unknown(&mut self, text: impl Into<String>) {
        self.default_unknown = text.into();
    }

    /// Clear dialog state
    pub fn reset(&mut self) {
        self.state.clear();
    }

    /// Get current topic
    pub fn current_topic(&self) -> Option<&String> {
        self.state.topic.as_ref()
    }
}

impl Default for DialogHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Pre-built dialog templates for common NPC types
pub struct DialogTemplates;

impl DialogTemplates {
    /// Create a banker dialog handler
    pub fn banker() -> DialogHandler {
        let mut handler = DialogHandler::new();
        handler.set_greeting("Welcome to the bank! I can {balance}, {deposit} or {withdraw} your gold.");

        handler.add_response(DialogResponse::new(
            vec!["balance"],
            "Your balance is {balance} gold coins.",
        ));

        handler.add_response(DialogResponse {
            keywords: vec!["deposit".to_string()],
            text: "How much would you like to deposit?".to_string(),
            set_topic: Some("deposit".to_string()),
            require_topic: None,
            once: false,
            conditions: HashMap::new(),
            set_vars: HashMap::new(),
            action: None,
        });

        handler.add_response(DialogResponse {
            keywords: vec!["withdraw".to_string()],
            text: "How much would you like to withdraw?".to_string(),
            set_topic: Some("withdraw".to_string()),
            require_topic: None,
            once: false,
            conditions: HashMap::new(),
            set_vars: HashMap::new(),
            action: None,
        });

        handler
    }

    /// Create a shop NPC dialog handler
    pub fn shop(shop_name: &str) -> DialogHandler {
        let mut handler = DialogHandler::new();
        handler.set_greeting(
            "Welcome! Do you want to see my {trade}? I also {buy} items.",
        );

        handler.add_response(DialogResponse {
            keywords: vec!["trade".to_string(), "wares".to_string(), "goods".to_string()],
            text: "Take a look at my wares.".to_string(),
            set_topic: None,
            require_topic: None,
            once: false,
            conditions: HashMap::new(),
            set_vars: HashMap::new(),
            action: Some(DialogAction::OpenShop(shop_name.to_string())),
        });

        handler.add_response(DialogResponse::new(
            vec!["buy", "sell"],
            "Just ask me about what you want to buy or sell.",
        ));

        handler
    }

    /// Create a spell trainer dialog handler
    pub fn spell_trainer(vocation: &str) -> DialogHandler {
        let mut handler = DialogHandler::new();
        handler.set_greeting(&format!(
            "Greetings, fellow {}! I can teach you powerful {{spells}}.",
            vocation
        ));

        handler.add_response(DialogResponse::new(
            vec!["spells", "teach", "learn"],
            "Which spell would you like to learn?",
        ));

        handler
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_state() {
        let mut state = DialogState::new();
        state.set_topic("trade");
        assert_eq!(state.topic.as_deref(), Some("trade"));

        state.set_var("player_name", "Test");
        assert_eq!(state.get_var("player_name"), Some(&"Test".to_string()));

        state.set_topic("confirm");
        assert_eq!(state.topic.as_deref(), Some("confirm"));
        
        state.pop_topic();
        assert_eq!(state.topic.as_deref(), Some("trade"));
    }

    #[test]
    fn test_greeting_detection() {
        let handler = DialogHandler::new();
        assert!(handler.is_greeting("hi"));
        assert!(handler.is_greeting("Hello there!"));
        assert!(!handler.is_greeting("how are you"));
    }

    #[test]
    fn test_response_matching() {
        let response = DialogResponse::new(vec!["trade", "buy"], "What would you like?");
        let state = DialogState::new();

        assert!(response.matches("I want to trade", &state));
        assert!(response.matches("can I buy something", &state));
        assert!(!response.matches("hello", &state));
    }

    #[test]
    fn test_dialog_processing() {
        let mut handler = DialogHandler::new();
        handler.add_response(DialogResponse::new(
            vec!["job"],
            "I am a merchant.",
        ));

        let response = handler.process_message("what is your job?");
        assert_eq!(response, Some("I am a merchant.".to_string()));

        let unknown = handler.process_message("random text");
        assert_eq!(unknown, Some("I don't understand what you mean.".to_string()));
    }
}
