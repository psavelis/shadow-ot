//! Lua Scripting Engine
//!
//! Provides Lua scripting capabilities for custom game logic.

use mlua::{Lua, Result as LuaResult, Table, Function, Value};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{Result, ScriptError};

/// The Lua scripting engine
pub struct LuaEngine {
    lua: Lua,
    scripts: HashMap<String, String>,
}

impl LuaEngine {
    /// Create a new Lua engine
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        
        // Set up sandbox (disable dangerous functions)
        lua.scope(|scope| {
            // Could set up metatables and sandboxing here
            Ok(())
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;

        Ok(Self {
            lua,
            scripts: HashMap::new(),
        })
    }

    /// Register game API functions
    pub fn register_api(&self) -> Result<()> {
        let globals = self.lua.globals();

        // Create Game namespace
        let game_table = self.lua.create_table()
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Game.sendMessage
        let send_message = self.lua.create_function(|_, (player_id, message): (String, String)| {
            tracing::debug!("Lua: sendMessage to {} - {}", player_id, message);
            Ok(())
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;
        game_table.set("sendMessage", send_message)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Game.teleport
        let teleport = self.lua.create_function(|_, (player_id, x, y, z): (String, u16, u16, u8)| {
            tracing::debug!("Lua: teleport {} to {},{},{}", player_id, x, y, z);
            Ok(())
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;
        game_table.set("teleport", teleport)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Game.giveItem
        let give_item = self.lua.create_function(|_, (player_id, item_id, count): (String, u16, u16)| {
            tracing::debug!("Lua: giveItem to {} - {} x{}", player_id, item_id, count);
            Ok(true)
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;
        game_table.set("giveItem", give_item)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Game.getStorage
        let get_storage = self.lua.create_function(|_, (player_id, key): (String, u32)| {
            tracing::debug!("Lua: getStorage {} key {}", player_id, key);
            Ok(0i32)
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;
        game_table.set("getStorage", get_storage)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Game.setStorage
        let set_storage = self.lua.create_function(|_, (player_id, key, value): (String, u32, i32)| {
            tracing::debug!("Lua: setStorage {} key {} = {}", player_id, key, value);
            Ok(())
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;
        game_table.set("setStorage", set_storage)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        globals.set("Game", game_table)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Create Player namespace
        let player_table = self.lua.create_table()
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Player.getLevel
        let get_level = self.lua.create_function(|_, player_id: String| {
            tracing::debug!("Lua: getLevel for {}", player_id);
            Ok(100u16)
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;
        player_table.set("getLevel", get_level)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Player.getHealth
        let get_health = self.lua.create_function(|_, player_id: String| {
            Ok((100i32, 100i32)) // current, max
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;
        player_table.set("getHealth", get_health)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Player.setHealth
        let set_health = self.lua.create_function(|_, (player_id, health): (String, i32)| {
            tracing::debug!("Lua: setHealth {} = {}", player_id, health);
            Ok(())
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;
        player_table.set("setHealth", set_health)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        globals.set("Player", player_table)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Create Creature namespace
        let creature_table = self.lua.create_table()
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Creature.spawn
        let spawn = self.lua.create_function(|_, (name, x, y, z): (String, u16, u16, u8)| {
            tracing::debug!("Lua: spawn {} at {},{},{}", name, x, y, z);
            Ok(0u32) // creature id
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;
        creature_table.set("spawn", spawn)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        globals.set("Creature", creature_table)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Utility functions
        let print_fn = self.lua.create_function(|_, message: String| {
            tracing::info!("[Lua] {}", message);
            Ok(())
        }).map_err(|e| ScriptError::Lua(e.to_string()))?;
        globals.set("print", print_fn)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        Ok(())
    }

    /// Load a script file
    pub fn load_file(&mut self, name: &str, path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        self.scripts.insert(name.to_string(), content.clone());

        self.lua.load(&content)
            .set_name(name)
            .exec()
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        tracing::debug!("Loaded Lua script: {}", name);
        Ok(())
    }

    /// Load script from string
    pub fn load_string(&mut self, name: &str, code: &str) -> Result<()> {
        self.scripts.insert(name.to_string(), code.to_string());

        self.lua.load(code)
            .set_name(name)
            .exec()
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        Ok(())
    }

    /// Execute a Lua string
    pub fn execute(&self, code: &str) -> Result<()> {
        self.lua.load(code)
            .exec()
            .map_err(|e| ScriptError::Lua(e.to_string()))?;
        Ok(())
    }

    /// Call a global function
    pub fn call_function(&self, name: &str, args: Vec<LuaValue>) -> Result<LuaValue> {
        let globals = self.lua.globals();
        let func: Function = globals.get(name)
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        // Convert args to mlua Values
        let lua_args: Vec<Value> = args.into_iter()
            .map(|v| self.to_lua_value(v))
            .collect::<Result<Vec<_>>>()?;

        let result: Value = func.call(mlua::MultiValue::from_vec(lua_args))
            .map_err(|e| ScriptError::Lua(e.to_string()))?;

        self.from_lua_value(result)
    }

    /// Convert our value type to Lua
    fn to_lua_value(&self, value: LuaValue) -> Result<Value> {
        match value {
            LuaValue::Nil => Ok(Value::Nil),
            LuaValue::Bool(b) => Ok(Value::Boolean(b)),
            LuaValue::Int(i) => Ok(Value::Integer(i)),
            LuaValue::Float(f) => Ok(Value::Number(f)),
            LuaValue::String(s) => {
                let lua_str = self.lua.create_string(&s)
                    .map_err(|e| ScriptError::Lua(e.to_string()))?;
                Ok(Value::String(lua_str))
            }
            LuaValue::Table(map) => {
                let table = self.lua.create_table()
                    .map_err(|e| ScriptError::Lua(e.to_string()))?;
                for (k, v) in map {
                    let lua_v = self.to_lua_value(v)?;
                    table.set(k, lua_v)
                        .map_err(|e| ScriptError::Lua(e.to_string()))?;
                }
                Ok(Value::Table(table))
            }
        }
    }

    /// Convert Lua value to our type
    fn from_lua_value(&self, value: Value) -> Result<LuaValue> {
        match value {
            Value::Nil => Ok(LuaValue::Nil),
            Value::Boolean(b) => Ok(LuaValue::Bool(b)),
            Value::Integer(i) => Ok(LuaValue::Int(i)),
            Value::Number(f) => Ok(LuaValue::Float(f)),
            Value::String(s) => Ok(LuaValue::String(s.to_str()
                .map_err(|e| ScriptError::Lua(e.to_string()))?
                .to_string())),
            Value::Table(t) => {
                let mut map = HashMap::new();
                for pair in t.pairs::<String, Value>() {
                    let (k, v) = pair.map_err(|e| ScriptError::Lua(e.to_string()))?;
                    map.insert(k, self.from_lua_value(v)?);
                }
                Ok(LuaValue::Table(map))
            }
            _ => Ok(LuaValue::Nil),
        }
    }

    /// Load all scripts from a directory
    pub fn load_directory(&mut self, path: &Path) -> Result<usize> {
        let mut count = 0;

        if !path.exists() {
            return Ok(0);
        }

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |e| e == "lua") {
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                
                self.load_file(name, &path)?;
                count += 1;
            }
        }

        tracing::info!("Loaded {} Lua scripts from {:?}", count, path);
        Ok(count)
    }
}

impl Default for LuaEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create Lua engine")
    }
}

/// Value types that can be passed to/from Lua
#[derive(Debug, Clone)]
pub enum LuaValue {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Table(HashMap<String, LuaValue>),
}

impl From<bool> for LuaValue {
    fn from(b: bool) -> Self {
        LuaValue::Bool(b)
    }
}

impl From<i32> for LuaValue {
    fn from(i: i32) -> Self {
        LuaValue::Int(i as i64)
    }
}

impl From<i64> for LuaValue {
    fn from(i: i64) -> Self {
        LuaValue::Int(i)
    }
}

impl From<f64> for LuaValue {
    fn from(f: f64) -> Self {
        LuaValue::Float(f)
    }
}

impl From<String> for LuaValue {
    fn from(s: String) -> Self {
        LuaValue::String(s)
    }
}

impl From<&str> for LuaValue {
    fn from(s: &str) -> Self {
        LuaValue::String(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lua_engine_creation() {
        let engine = LuaEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_lua_execute() {
        let engine = LuaEngine::new().unwrap();
        let result = engine.execute("x = 1 + 1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_lua_api_registration() {
        let engine = LuaEngine::new().unwrap();
        let result = engine.register_api();
        assert!(result.is_ok());
    }
}
