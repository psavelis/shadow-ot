--[[
    Shadow OT Discord Rich Presence Integration

    Shows game status on Discord with rich presence features:
    - Current realm and character
    - Level and vocation
    - Online time
    - Party info
    - Guild info
]]

Discord = {}

-- Discord Application ID (would be real Shadow OT app ID)
Discord.applicationId = "1234567890123456789"

-- Presence state
Discord.presence = {
    state = "In Menu",
    details = "Shadow OT Client",
    largeImageKey = "shadow_ot_logo",
    largeImageText = "Shadow OT",
    smallImageKey = nil,
    smallImageText = nil,
    startTimestamp = nil,
    partyId = nil,
    partySize = 0,
    partyMax = 0
}

-- Configuration
Discord.config = {
    enabled = true,
    showCharacter = true,
    showLevel = true,
    showRealm = true,
    showParty = true,
    showGuild = true,
    streamingMode = false
}

-- Realm image keys
Discord.realmImages = {
    ["Mythara"] = "realm_mythara",
    ["Aetheria"] = "realm_aetheria",
    ["Shadowveil"] = "realm_shadowveil",
    ["Voidborne"] = "realm_voidborne",
    ["Warbound"] = "realm_warbound",
    ["Astralforge"] = "realm_astralforge"
}

-- Vocation image keys
Discord.vocationImages = {
    ["Knight"] = "vocation_knight",
    ["Paladin"] = "vocation_paladin",
    ["Sorcerer"] = "vocation_sorcerer",
    ["Druid"] = "vocation_druid",
    ["Elite Knight"] = "vocation_elite_knight",
    ["Royal Paladin"] = "vocation_royal_paladin",
    ["Master Sorcerer"] = "vocation_master_sorcerer",
    ["Elder Druid"] = "vocation_elder_druid"
}

-- Initialize Discord module
function Discord.init()
    print("[Discord] Rich Presence module initialized")

    -- Connect to game events
    connect(g_game, {
        onGameStart = Discord.onGameStart,
        onGameEnd = Discord.onGameEnd
    })

    -- Start presence update loop
    Discord.startPresence()
end

-- Terminate Discord module
function Discord.terminate()
    Discord.clearPresence()

    disconnect(g_game, {
        onGameStart = Discord.onGameStart,
        onGameEnd = Discord.onGameEnd
    })
end

-- Start Discord presence
function Discord.startPresence()
    if not Discord.config.enabled then return end

    -- Initialize Discord SDK (would use Discord Game SDK)
    -- For now, simulate the connection
    print("[Discord] Connected to Discord")

    Discord.presence.startTimestamp = os.time()
    Discord.updatePresence()
end

-- Clear Discord presence
function Discord.clearPresence()
    -- Clear rich presence
    print("[Discord] Presence cleared")
end

-- Update presence based on game state
function Discord.updatePresence()
    if not Discord.config.enabled then return end

    local player = g_game.getLocalPlayer()

    if not player or not g_game.isOnline() then
        Discord.setMenuPresence()
    else
        Discord.setInGamePresence(player)
    end

    -- Schedule next update
    scheduleEvent(Discord.updatePresence, 15000) -- Update every 15 seconds
end

-- Set menu/offline presence
function Discord.setMenuPresence()
    Discord.presence.state = "In Menu"
    Discord.presence.details = "Browsing realms"
    Discord.presence.smallImageKey = nil
    Discord.presence.smallImageText = nil
    Discord.presence.partyId = nil
    Discord.presence.partySize = 0
    Discord.presence.partyMax = 0

    Discord.sendPresence()
end

-- Set in-game presence
function Discord.setInGamePresence(player)
    local name = player:getName()
    local level = player:getLevel()
    local vocation = player:getVocationName()

    -- Get realm info
    local realm = Discord.getCurrentRealm()

    -- Build state string
    local state = ""
    if Discord.config.showLevel then
        state = "Level " .. level .. " " .. vocation
    else
        state = "Playing"
    end

    -- Build details string
    local details = ""
    if Discord.config.showCharacter and not Discord.config.streamingMode then
        details = name
    else
        details = "Shadow OT"
    end

    if Discord.config.showRealm and realm then
        details = details .. " on " .. realm
    end

    -- Set presence
    Discord.presence.state = state
    Discord.presence.details = details

    -- Set realm image
    if realm and Discord.realmImages[realm] then
        Discord.presence.largeImageKey = Discord.realmImages[realm]
        Discord.presence.largeImageText = realm .. " Realm"
    else
        Discord.presence.largeImageKey = "shadow_ot_logo"
        Discord.presence.largeImageText = "Shadow OT"
    end

    -- Set vocation image
    if vocation and Discord.vocationImages[vocation] then
        Discord.presence.smallImageKey = Discord.vocationImages[vocation]
        Discord.presence.smallImageText = vocation
    end

    -- Party info
    if Discord.config.showParty then
        local party = Discord.getPartyInfo()
        if party then
            Discord.presence.partyId = party.id
            Discord.presence.partySize = party.size
            Discord.presence.partyMax = party.max
        end
    end

    Discord.sendPresence()
end

-- Get current realm name
function Discord.getCurrentRealm()
    -- Would get from game state
    return "Mythara"
end

-- Get party info
function Discord.getPartyInfo()
    -- Would get from game state
    return nil
end

-- Send presence to Discord
function Discord.sendPresence()
    -- Would call Discord Game SDK here
    -- Discord_UpdatePresence(Discord.presence)

    if Discord.config.enabled then
        print(string.format("[Discord] Presence: %s - %s",
            Discord.presence.details,
            Discord.presence.state))
    end
end

-- Game event handlers
function Discord.onGameStart()
    Discord.presence.startTimestamp = os.time()
    Discord.updatePresence()
end

function Discord.onGameEnd()
    Discord.setMenuPresence()
end

-- Configuration functions

function Discord.enable()
    Discord.config.enabled = true
    Discord.updatePresence()
end

function Discord.disable()
    Discord.config.enabled = false
    Discord.clearPresence()
end

function Discord.setStreamingMode(enabled)
    Discord.config.streamingMode = enabled
    Discord.updatePresence()
end

function Discord.setShowCharacter(enabled)
    Discord.config.showCharacter = enabled
    Discord.updatePresence()
end

function Discord.setShowLevel(enabled)
    Discord.config.showLevel = enabled
    Discord.updatePresence()
end

function Discord.setShowRealm(enabled)
    Discord.config.showRealm = enabled
    Discord.updatePresence()
end

function Discord.setShowParty(enabled)
    Discord.config.showParty = enabled
    Discord.updatePresence()
end

function Discord.setShowGuild(enabled)
    Discord.config.showGuild = enabled
    Discord.updatePresence()
end

-- Get elapsed time string
function Discord.getElapsedTime()
    if not Discord.presence.startTimestamp then return "0:00" end

    local elapsed = os.time() - Discord.presence.startTimestamp
    local hours = math.floor(elapsed / 3600)
    local minutes = math.floor((elapsed % 3600) / 60)
    local seconds = elapsed % 60

    if hours > 0 then
        return string.format("%d:%02d:%02d", hours, minutes, seconds)
    else
        return string.format("%d:%02d", minutes, seconds)
    end
end

return Discord
