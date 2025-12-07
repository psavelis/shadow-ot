-- Shadow OT Modules Initialization
-- This file orchestrates loading of all client modules

print("[Shadow OT] Starting module initialization...")

-- Client main module (login screen)
local function loadModule(name, file)
    local success, err = pcall(function()
        dofile(name .. '/' .. file)
    end)
    if success then
        print("[Shadow OT] Loaded: " .. name)
    else
        print("[Shadow OT] Warning: Could not load " .. name .. " - " .. tostring(err))
    end
end

-- Load client main first (login screen)
loadModule('client_main', 'client_main.lua')

-- Core game modules (loaded after login)
loadModule('game_battle', 'battle.lua')
loadModule('game_console', 'console.lua')
loadModule('game_containers', 'containers.lua')
loadModule('game_interface', 'gameinterface.lua')
loadModule('game_inventory', 'inventory.lua')
loadModule('game_minimap', 'minimap.lua')
loadModule('game_skills', 'skills.lua')

-- Shadow OT specific modules
loadModule('shadow_realms', 'realms.lua')
loadModule('shadow_blockchain', 'blockchain.lua')
loadModule('shadow_bot', 'bot.lua')
loadModule('shadow_discord', 'discord.lua')
loadModule('shadow_streaming', 'streaming.lua')
loadModule('shadow_updater', 'updater.lua')

print("[Shadow OT] All modules loaded successfully")
