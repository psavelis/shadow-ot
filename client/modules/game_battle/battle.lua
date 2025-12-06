-- Shadow OT Battle Module
-- Handles the battle/target list window

Battle = {}

local battleWindow = nil
local battleList = nil
local filterPanel = nil
local selectedCreature = nil
local creatures = {}

-- Filter settings
local filters = {
    showPlayers = true,
    showNPCs = true,
    showMonsters = true,
    showSkulled = true,
    sortBy = 'distance' -- 'distance', 'health', 'name'
}

function Battle.init()
    connect(g_game, {
        onGameStart = Battle.onGameStart,
        onGameEnd = Battle.onGameEnd,
        onCreatureAppear = Battle.onCreatureAppear,
        onCreatureDisappear = Battle.onCreatureDisappear,
        onCreatureHealthPercentChange = Battle.onCreatureHealthChange,
        onCreatureSkullChange = Battle.onCreatureSkullChange,
        onAttackingCreatureChange = Battle.onAttackingCreatureChange,
        onFollowingCreatureChange = Battle.onFollowingCreatureChange
    })

    battleWindow = g_ui.displayUI('battle')
    battleList = battleWindow:getChildById('battleList')
    filterPanel = battleWindow:getChildById('filterPanel')

    -- Setup filter buttons
    Battle.setupFilters()

    battleWindow:hide()
end

function Battle.terminate()
    disconnect(g_game, {
        onGameStart = Battle.onGameStart,
        onGameEnd = Battle.onGameEnd,
        onCreatureAppear = Battle.onCreatureAppear,
        onCreatureDisappear = Battle.onCreatureDisappear,
        onCreatureHealthPercentChange = Battle.onCreatureHealthChange,
        onCreatureSkullChange = Battle.onCreatureSkullChange,
        onAttackingCreatureChange = Battle.onAttackingCreatureChange,
        onFollowingCreatureChange = Battle.onFollowingCreatureChange
    })

    if battleWindow then
        battleWindow:destroy()
        battleWindow = nil
    end
end

function Battle.setupFilters()
    local playerBtn = filterPanel:getChildById('filterPlayers')
    local npcBtn = filterPanel:getChildById('filterNPCs')
    local monsterBtn = filterPanel:getChildById('filterMonsters')
    local skulledBtn = filterPanel:getChildById('filterSkulled')

    playerBtn.onClick = function()
        filters.showPlayers = not filters.showPlayers
        playerBtn:setChecked(filters.showPlayers)
        Battle.refresh()
    end

    npcBtn.onClick = function()
        filters.showNPCs = not filters.showNPCs
        npcBtn:setChecked(filters.showNPCs)
        Battle.refresh()
    end

    monsterBtn.onClick = function()
        filters.showMonsters = not filters.showMonsters
        monsterBtn:setChecked(filters.showMonsters)
        Battle.refresh()
    end

    skulledBtn.onClick = function()
        filters.showSkulled = not filters.showSkulled
        skulledBtn:setChecked(filters.showSkulled)
        Battle.refresh()
    end
end

function Battle.onGameStart()
    battleWindow:show()
    creatures = {}
    Battle.refresh()
end

function Battle.onGameEnd()
    battleWindow:hide()
    creatures = {}
    Battle.clear()
end

function Battle.onCreatureAppear(creature)
    if creature == g_game.getLocalPlayer() then return end

    creatures[creature:getId()] = creature
    Battle.addCreature(creature)
end

function Battle.onCreatureDisappear(creature)
    creatures[creature:getId()] = nil
    Battle.removeCreature(creature:getId())
end

function Battle.onCreatureHealthChange(creature, healthPercent)
    Battle.updateCreature(creature)
end

function Battle.onCreatureSkullChange(creature, skull)
    Battle.updateCreature(creature)
end

function Battle.onAttackingCreatureChange(creature, oldCreature)
    if oldCreature then
        Battle.setCreatureAttacked(oldCreature:getId(), false)
    end
    if creature then
        Battle.setCreatureAttacked(creature:getId(), true)
    end
end

function Battle.onFollowingCreatureChange(creature, oldCreature)
    if oldCreature then
        Battle.setCreatureFollowed(oldCreature:getId(), false)
    end
    if creature then
        Battle.setCreatureFollowed(creature:getId(), true)
    end
end

function Battle.shouldShowCreature(creature)
    if creature:isPlayer() and not filters.showPlayers then return false end
    if creature:isNpc() and not filters.showNPCs then return false end
    if creature:isMonster() and not filters.showMonsters then return false end

    if not filters.showSkulled then
        local skull = creature:getSkull()
        if skull ~= 0 then return false end
    end

    return true
end

function Battle.addCreature(creature)
    if not Battle.shouldShowCreature(creature) then return end

    local widget = g_ui.createWidget('BattleEntry', battleList)
    widget.creatureId = creature:getId()

    Battle.setupBattleEntry(widget, creature)
    Battle.sortList()
end

function Battle.setupBattleEntry(widget, creature)
    -- Creature outfit display
    local outfitWidget = widget:getChildById('creatureOutfit')
    outfitWidget:setCreature(creature)

    -- Name
    widget:getChildById('creatureName'):setText(creature:getName())

    -- Health bar
    local healthBar = widget:getChildById('healthBar')
    healthBar:setPercent(creature:getHealthPercent())
    healthBar:setBackgroundColor(Battle.getHealthColor(creature:getHealthPercent()))

    -- Skull icon
    local skullIcon = widget:getChildById('skullIcon')
    local skull = creature:getSkull()
    if skull > 0 then
        skullIcon:setImageSource('/images/game/skulls/' .. skull)
        skullIcon:setVisible(true)
    else
        skullIcon:setVisible(false)
    end

    -- Click handlers
    widget.onClick = function()
        Battle.selectCreature(widget)
    end

    widget.onDoubleClick = function()
        g_game.attack(creature:getId())
    end

    widget.onMouseRelease = function(self, mousePos, mouseButton)
        if mouseButton == 2 then -- Right click
            Battle.showContextMenu(creature, mousePos)
        end
    end
end

function Battle.removeCreature(creatureId)
    for _, child in ipairs(battleList:getChildren()) do
        if child.creatureId == creatureId then
            child:destroy()
            break
        end
    end
end

function Battle.updateCreature(creature)
    for _, child in ipairs(battleList:getChildren()) do
        if child.creatureId == creature:getId() then
            local healthBar = child:getChildById('healthBar')
            healthBar:setPercent(creature:getHealthPercent())
            healthBar:setBackgroundColor(Battle.getHealthColor(creature:getHealthPercent()))

            local skullIcon = child:getChildById('skullIcon')
            local skull = creature:getSkull()
            if skull > 0 then
                skullIcon:setImageSource('/images/game/skulls/' .. skull)
                skullIcon:setVisible(true)
            else
                skullIcon:setVisible(false)
            end
            break
        end
    end
end

function Battle.setCreatureAttacked(creatureId, attacked)
    for _, child in ipairs(battleList:getChildren()) do
        if child.creatureId == creatureId then
            if attacked then
                child:setBackgroundColor('#803030')
            else
                child:setBackgroundColor('#2d2d2d')
            end
            break
        end
    end
end

function Battle.setCreatureFollowed(creatureId, followed)
    for _, child in ipairs(battleList:getChildren()) do
        if child.creatureId == creatureId then
            local followIcon = child:getChildById('followIcon')
            followIcon:setVisible(followed)
            break
        end
    end
end

function Battle.selectCreature(widget)
    if selectedCreature then
        selectedCreature:setBackgroundColor('#2d2d2d')
    end
    selectedCreature = widget
    widget:setBackgroundColor('#3d5a80')
end

function Battle.showContextMenu(creature, mousePos)
    local menu = g_ui.createWidget('PopupMenu')

    menu:addOption('Attack', function()
        g_game.attack(creature:getId())
    end)

    menu:addOption('Follow', function()
        g_game.follow(creature:getId())
    end)

    menu:addSeparator()

    menu:addOption('Look', function()
        g_game.look(creature:getPosition(), creature:getId(), 0)
    end)

    if creature:isPlayer() then
        menu:addOption('Message', function()
            modules.game_console.openPrivateChannel(creature:getName())
        end)

        menu:addOption('Add to VIP', function()
            g_game.addVip(creature:getName())
        end)

        menu:addSeparator()

        menu:addOption('Invite to Party', function()
            g_game.inviteToParty(creature:getId())
        end)
    end

    menu:display(mousePos)
end

function Battle.getHealthColor(percent)
    if percent > 92 then
        return '#00bc00'
    elseif percent > 60 then
        return '#50a150'
    elseif percent > 30 then
        return '#a1a100'
    elseif percent > 8 then
        return '#bf0a0a'
    elseif percent > 3 then
        return '#910f0f'
    else
        return '#3d0000'
    end
end

function Battle.sortList()
    local children = battleList:getChildren()
    local player = g_game.getLocalPlayer()
    local playerPos = player:getPosition()

    table.sort(children, function(a, b)
        local creatureA = creatures[a.creatureId]
        local creatureB = creatures[b.creatureId]
        if not creatureA or not creatureB then return false end

        if filters.sortBy == 'distance' then
            local distA = playerPos:getDistance(creatureA:getPosition())
            local distB = playerPos:getDistance(creatureB:getPosition())
            return distA < distB
        elseif filters.sortBy == 'health' then
            return creatureA:getHealthPercent() < creatureB:getHealthPercent()
        else -- name
            return creatureA:getName() < creatureB:getName()
        end
    end)

    -- Re-add sorted children
    battleList:destroyChildren()
    for _, child in ipairs(children) do
        battleList:addChild(child)
    end
end

function Battle.refresh()
    Battle.clear()
    for _, creature in pairs(creatures) do
        Battle.addCreature(creature)
    end
end

function Battle.clear()
    battleList:destroyChildren()
    selectedCreature = nil
end

function Battle.toggle()
    if battleWindow:isVisible() then
        battleWindow:hide()
    else
        battleWindow:show()
        battleWindow:raise()
    end
end

function Battle.show()
    battleWindow:show()
    battleWindow:raise()
end

function Battle.hide()
    battleWindow:hide()
end
