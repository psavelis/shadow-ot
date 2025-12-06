--[[
    Shadow OT Bot Framework

    Provides authorized botting features for designated bot zones.
    Features: Auto-heal, Auto-attack, Cavebot, Looting, Training

    Usage:
    Bot.onHealthLow(function(percent)
        if percent < 50 then
            Bot.useItem("supreme health potion")
        end
    end)
]]

Bot = {}

-- Bot configuration
Bot.config = {
    enabled = false,
    zone = nil, -- Current bot zone (nil = unauthorized)
    autoHeal = {
        enabled = false,
        healthPercent = 50,
        manaPercent = 30,
        healthItem = "supreme health potion",
        manaItem = "great mana potion"
    },
    autoAttack = {
        enabled = false,
        targetList = {},
        mode = "closest", -- closest, lowest_hp, highest_hp
        safeDistance = 1
    },
    cavebot = {
        enabled = false,
        waypoints = {},
        currentIndex = 1,
        looting = true,
        respawnTime = 60
    },
    training = {
        enabled = false,
        skill = nil,
        target = nil
    }
}

-- Bot state
Bot.state = {
    running = false,
    lastHeal = 0,
    lastManaHeal = 0,
    lastAttack = 0,
    lastLoot = 0,
    currentTarget = nil,
    walkPath = {}
}

-- Callbacks
Bot.callbacks = {
    onHealthLow = nil,
    onManaLow = nil,
    onTarget = nil,
    onLoot = nil,
    onWaypoint = nil,
    onDeath = nil,
    onZoneEnter = nil,
    onZoneLeave = nil
}

-- Bot zones (authorized areas for botting)
Bot.zones = {
    -- Training grounds
    { name = "Thais Training Hall", x1 = 32300, y1 = 32200, x2 = 32350, y2 = 32250, z = 7 },
    { name = "Carlin Training Grounds", x1 = 32400, y1 = 31700, x2 = 32450, y2 = 31750, z = 7 },
    -- Bot hunting zones
    { name = "Dragon Lair Bot Zone", x1 = 32800, y1 = 32500, x2 = 32900, y2 = 32600, z = 8 },
    { name = "Demon Spawn Bot Zone", x1 = 33000, y1 = 31500, x2 = 33100, y2 = 31600, z = 10 }
}

-- Initialize bot module
function Bot.init()
    print("[Bot] Shadow OT Bot Framework initialized")

    -- Register event handlers
    connect(g_game, {
        onGameStart = Bot.onGameStart,
        onGameEnd = Bot.onGameEnd
    })

    -- Create bot panel UI
    Bot.createPanel()
end

-- Terminate bot module
function Bot.terminate()
    Bot.stop()

    disconnect(g_game, {
        onGameStart = Bot.onGameStart,
        onGameEnd = Bot.onGameEnd
    })

    if Bot.panel then
        Bot.panel:destroy()
        Bot.panel = nil
    end
end

-- Create bot control panel
function Bot.createPanel()
    -- Panel would be created using OTUI
    -- Simplified for now
    print("[Bot] Control panel ready")
end

-- Game event handlers
function Bot.onGameStart()
    -- Check if player is in authorized zone
    Bot.checkZone()
end

function Bot.onGameEnd()
    Bot.stop()
end

-- Check if player is in authorized bot zone
function Bot.checkZone()
    local player = g_game.getLocalPlayer()
    if not player then return false end

    local pos = player:getPosition()

    for _, zone in ipairs(Bot.zones) do
        if pos.x >= zone.x1 and pos.x <= zone.x2 and
           pos.y >= zone.y1 and pos.y <= zone.y2 and
           pos.z == zone.z then
            Bot.config.zone = zone
            if Bot.callbacks.onZoneEnter then
                Bot.callbacks.onZoneEnter(zone)
            end
            return true
        end
    end

    if Bot.config.zone then
        local oldZone = Bot.config.zone
        Bot.config.zone = nil
        if Bot.callbacks.onZoneLeave then
            Bot.callbacks.onZoneLeave(oldZone)
        end
    end

    return false
end

-- Start bot
function Bot.start()
    if not Bot.checkZone() then
        print("[Bot] Error: You must be in an authorized bot zone!")
        return false
    end

    Bot.state.running = true
    Bot.config.enabled = true

    -- Start bot loop
    scheduleEvent(Bot.loop, 100)

    print("[Bot] Bot started in zone: " .. Bot.config.zone.name)
    return true
end

-- Stop bot
function Bot.stop()
    Bot.state.running = false
    Bot.config.enabled = false
    Bot.state.currentTarget = nil
    Bot.state.walkPath = {}

    print("[Bot] Bot stopped")
end

-- Main bot loop
function Bot.loop()
    if not Bot.state.running then return end
    if not Bot.checkZone() then
        Bot.stop()
        print("[Bot] Left authorized zone - stopping bot")
        return
    end

    local player = g_game.getLocalPlayer()
    if not player then return end

    -- Auto-heal check
    if Bot.config.autoHeal.enabled then
        Bot.checkHeal(player)
    end

    -- Auto-attack check
    if Bot.config.autoAttack.enabled then
        Bot.checkAttack(player)
    end

    -- Cavebot waypoint check
    if Bot.config.cavebot.enabled then
        Bot.checkWaypoint(player)
    end

    -- Schedule next loop iteration
    if Bot.state.running then
        scheduleEvent(Bot.loop, 100)
    end
end

-- Auto-heal logic
function Bot.checkHeal(player)
    local now = os.time()
    local healthPercent = (player:getHealth() / player:getMaxHealth()) * 100
    local manaPercent = (player:getMana() / player:getMaxMana()) * 100

    -- Health potion
    if healthPercent < Bot.config.autoHeal.healthPercent then
        if now - Bot.state.lastHeal >= 1 then
            if Bot.callbacks.onHealthLow then
                Bot.callbacks.onHealthLow(healthPercent)
            else
                Bot.useItem(Bot.config.autoHeal.healthItem)
            end
            Bot.state.lastHeal = now
        end
    end

    -- Mana potion
    if manaPercent < Bot.config.autoHeal.manaPercent then
        if now - Bot.state.lastManaHeal >= 1 then
            if Bot.callbacks.onManaLow then
                Bot.callbacks.onManaLow(manaPercent)
            else
                Bot.useItem(Bot.config.autoHeal.manaItem)
            end
            Bot.state.lastManaHeal = now
        end
    end
end

-- Auto-attack logic
function Bot.checkAttack(player)
    if Bot.state.currentTarget then
        -- Check if target is still valid
        local creature = g_game.getCreatureById(Bot.state.currentTarget)
        if not creature or creature:getHealthPercent() <= 0 then
            Bot.state.currentTarget = nil
        end
    end

    if not Bot.state.currentTarget then
        -- Find new target
        local target = Bot.findTarget(player)
        if target then
            Bot.state.currentTarget = target:getCreatureId()
            g_game.attack(target:getCreatureId())

            if Bot.callbacks.onTarget then
                Bot.callbacks.onTarget(target)
            end
        end
    end
end

-- Find target based on mode
function Bot.findTarget(player)
    local playerPos = player:getPosition()
    local bestTarget = nil
    local bestValue = nil

    -- Get creatures in range (would use g_map.getSpectators in real implementation)
    local creatures = Bot.getNearbyCreatures(playerPos, 7)

    for _, creature in ipairs(creatures) do
        -- Check if creature is in target list
        if Bot.isValidTarget(creature) then
            local value = Bot.getTargetValue(creature, playerPos)

            if not bestTarget or value < bestValue then
                bestTarget = creature
                bestValue = value
            end
        end
    end

    return bestTarget
end

-- Check if creature is valid target
function Bot.isValidTarget(creature)
    if #Bot.config.autoAttack.targetList == 0 then
        -- Attack any monster if no target list
        return not creature:isPlayer()
    end

    local name = creature:getName():lower()
    for _, targetName in ipairs(Bot.config.autoAttack.targetList) do
        if name == targetName:lower() then
            return true
        end
    end

    return false
end

-- Get target priority value
function Bot.getTargetValue(creature, playerPos)
    local pos = creature:getPosition()
    local distance = math.abs(pos.x - playerPos.x) + math.abs(pos.y - playerPos.y)

    if Bot.config.autoAttack.mode == "closest" then
        return distance
    elseif Bot.config.autoAttack.mode == "lowest_hp" then
        return creature:getHealthPercent() + distance * 0.1
    elseif Bot.config.autoAttack.mode == "highest_hp" then
        return (100 - creature:getHealthPercent()) + distance * 0.1
    end

    return distance
end

-- Get nearby creatures (simplified)
function Bot.getNearbyCreatures(pos, range)
    -- This would use g_map.getSpectators in full implementation
    return {}
end

-- Cavebot waypoint logic
function Bot.checkWaypoint(player)
    if #Bot.config.cavebot.waypoints == 0 then return end

    local currentWp = Bot.config.cavebot.waypoints[Bot.config.cavebot.currentIndex]
    if not currentWp then return end

    local playerPos = player:getPosition()
    local distance = math.abs(playerPos.x - currentWp.x) + math.abs(playerPos.y - currentWp.y)

    if distance <= 1 and playerPos.z == currentWp.z then
        -- Reached waypoint
        if Bot.callbacks.onWaypoint then
            Bot.callbacks.onWaypoint(currentWp, Bot.config.cavebot.currentIndex)
        end

        -- Execute waypoint action
        if currentWp.action then
            currentWp.action()
        end

        -- Move to next waypoint
        Bot.config.cavebot.currentIndex = Bot.config.cavebot.currentIndex + 1
        if Bot.config.cavebot.currentIndex > #Bot.config.cavebot.waypoints then
            Bot.config.cavebot.currentIndex = 1 -- Loop back
        end
    else
        -- Walk towards waypoint
        Bot.walkTo(currentWp)
    end
end

-- Walk to position
function Bot.walkTo(pos)
    local player = g_game.getLocalPlayer()
    if not player then return end

    -- Use auto-walk
    g_game.autoWalk({Position(pos.x, pos.y, pos.z)})
end

-- Use item by name
function Bot.useItem(itemName)
    -- Would search inventory and use item
    print("[Bot] Using item: " .. itemName)
end

-- Callback setters (PRD examples)
function Bot.onHealthLow(callback)
    Bot.callbacks.onHealthLow = callback
end

function Bot.onManaLow(callback)
    Bot.callbacks.onManaLow = callback
end

function Bot.onTarget(callback)
    Bot.callbacks.onTarget = callback
end

function Bot.onLoot(callback)
    Bot.callbacks.onLoot = callback
end

function Bot.onWaypoint(callback)
    Bot.callbacks.onWaypoint = callback
end

function Bot.onDeath(callback)
    Bot.callbacks.onDeath = callback
end

-- Configure auto-heal
function Bot.setAutoHeal(config)
    if config.enabled ~= nil then
        Bot.config.autoHeal.enabled = config.enabled
    end
    if config.healthPercent then
        Bot.config.autoHeal.healthPercent = config.healthPercent
    end
    if config.manaPercent then
        Bot.config.autoHeal.manaPercent = config.manaPercent
    end
    if config.healthItem then
        Bot.config.autoHeal.healthItem = config.healthItem
    end
    if config.manaItem then
        Bot.config.autoHeal.manaItem = config.manaItem
    end
end

-- Configure auto-attack
function Bot.setAutoAttack(config)
    if config.enabled ~= nil then
        Bot.config.autoAttack.enabled = config.enabled
    end
    if config.targetList then
        Bot.config.autoAttack.targetList = config.targetList
    end
    if config.mode then
        Bot.config.autoAttack.mode = config.mode
    end
end

-- Add cavebot waypoint
function Bot.addWaypoint(x, y, z, action)
    table.insert(Bot.config.cavebot.waypoints, {
        x = x,
        y = y,
        z = z,
        action = action
    })
end

-- Clear cavebot waypoints
function Bot.clearWaypoints()
    Bot.config.cavebot.waypoints = {}
    Bot.config.cavebot.currentIndex = 1
end

-- Cast spell
function Bot.cast(spellName)
    g_game.say(spellName)
end

-- Check if bot is enabled
function Bot.isEnabled()
    return Bot.config.enabled and Bot.state.running
end

-- Check if in bot zone
function Bot.isInZone()
    return Bot.config.zone ~= nil
end

-- Get current zone
function Bot.getZone()
    return Bot.config.zone
end

return Bot
