-- Shadow OT Game Interface Module
-- Main game rendering and interaction

GameInterface = {}

local gameRootPanel = nil
local gameMapPanel = nil
local gameTopMenu = nil
local gameBottomPanel = nil
local gameLeftPanel = nil
local gameRightPanel = nil

local useWithItem = nil
local useWithPosition = nil

function GameInterface.init()
    connect(g_game, {
        onGameStart = GameInterface.onGameStart,
        onGameEnd = GameInterface.onGameEnd
    })

    connect(g_app, {
        onResize = GameInterface.onResize
    })

    gameRootPanel = g_ui.displayUI('gameinterface')
    gameMapPanel = gameRootPanel:getChildById('gameMapPanel')
    gameTopMenu = gameRootPanel:getChildById('topMenu')
    gameBottomPanel = gameRootPanel:getChildById('bottomPanel')
    gameLeftPanel = gameRootPanel:getChildById('leftPanel')
    gameRightPanel = gameRootPanel:getChildById('rightPanel')

    -- Setup top menu buttons
    GameInterface.setupTopMenu()

    -- Setup game map interactions
    GameInterface.setupMapPanel()

    gameRootPanel:hide()
end

function GameInterface.terminate()
    disconnect(g_game, {
        onGameStart = GameInterface.onGameStart,
        onGameEnd = GameInterface.onGameEnd
    })

    disconnect(g_app, {
        onResize = GameInterface.onResize
    })

    if gameRootPanel then
        gameRootPanel:destroy()
        gameRootPanel = nil
    end
end

function GameInterface.setupTopMenu()
    local menuButtons = {
        {id = 'logoutButton', text = 'Logout', action = function() g_game.logout() end},
        {id = 'optionsButton', text = 'Options', action = function() modules.client_options.show() end},
        {id = 'helpButton', text = 'Help', action = function() modules.client_help.show() end}
    }

    for _, btn in ipairs(menuButtons) do
        local button = gameTopMenu:getChildById(btn.id)
        if button then
            button.onClick = btn.action
        end
    end
end

function GameInterface.setupMapPanel()
    gameMapPanel.onMousePress = function(self, pos, button)
        local mapPos = GameInterface.getMapPosition(pos)
        if not mapPos then return false end

        if button == 1 then
            -- Left click
            if useWithItem then
                -- Complete use with
                GameInterface.completeUseWith(mapPos)
                return true
            else
                -- Click on map - walk or interact
                return GameInterface.onMapLeftClick(mapPos)
            end
        elseif button == 2 then
            -- Right click - context menu
            return GameInterface.onMapRightClick(mapPos, pos)
        end
        return false
    end

    gameMapPanel.onMouseMove = function(self, pos)
        local mapPos = GameInterface.getMapPosition(pos)
        if mapPos then
            GameInterface.updateMousePosition(mapPos)
        end
    end

    gameMapPanel.onMouseWheel = function(self, pos, delta)
        -- Could implement zoom here
        return false
    end
end

function GameInterface.onGameStart()
    gameRootPanel:show()
    gameRootPanel:raise()
    GameInterface.updateLayout()
end

function GameInterface.onGameEnd()
    gameRootPanel:hide()
    GameInterface.cancelUseWith()
end

function GameInterface.onResize(width, height)
    GameInterface.updateLayout()
end

function GameInterface.updateLayout()
    if not gameRootPanel then return end

    local rootRect = gameRootPanel:getRect()
    local topHeight = gameTopMenu:getHeight()
    local bottomHeight = gameBottomPanel:getHeight()
    local leftWidth = gameLeftPanel:getWidth()
    local rightWidth = gameRightPanel:getWidth()

    -- Update game map panel size
    gameMapPanel:setPosition(leftWidth, topHeight)
    gameMapPanel:setSize(
        rootRect.width - leftWidth - rightWidth,
        rootRect.height - topHeight - bottomHeight
    )
end

function GameInterface.getMapPosition(screenPos)
    return gameMapPanel:getMapPosition(screenPos)
end

function GameInterface.onMapLeftClick(mapPos)
    local player = g_game.getLocalPlayer()
    if not player then return false end

    local tile = g_map.getTile(mapPos)
    if not tile then
        -- Walk to position
        g_game.autoWalk({mapPos})
        return true
    end

    -- Check for creature first
    local creature = tile:getTopCreature()
    if creature and creature ~= player then
        if g_keyboard.isShiftPressed() then
            -- Look at creature
            g_game.look(mapPos, creature:getId(), 0)
        elseif g_keyboard.isCtrlPressed() then
            -- Attack creature
            g_game.attack(creature:getId())
        else
            -- Follow creature
            g_game.follow(creature:getId())
        end
        return true
    end

    -- Check for useable item
    local topItem = tile:getTopItem()
    if topItem then
        if g_keyboard.isCtrlPressed() then
            -- Use item
            g_game.use(mapPos, topItem:getId(), tile:getTopItemStackPos())
            return true
        elseif g_keyboard.isShiftPressed() then
            -- Look at item
            g_game.look(mapPos, topItem:getId(), tile:getTopItemStackPos())
            return true
        end
    end

    -- Default: walk to position
    g_game.autoWalk({mapPos})
    return true
end

function GameInterface.onMapRightClick(mapPos, screenPos)
    local tile = g_map.getTile(mapPos)
    local menu = g_ui.createWidget('PopupMenu')

    if tile then
        local creature = tile:getTopCreature()
        local topItem = tile:getTopItem()

        if creature then
            local player = g_game.getLocalPlayer()
            if creature ~= player then
                menu:addOption('Attack', function()
                    g_game.attack(creature:getId())
                end)
                menu:addOption('Follow', function()
                    g_game.follow(creature:getId())
                end)
                menu:addSeparator()
            end

            menu:addOption('Look', function()
                g_game.look(mapPos, creature:getId(), 0)
            end)

            if creature:isPlayer() and creature ~= player then
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
        end

        if topItem then
            if creature then
                menu:addSeparator()
            end

            menu:addOption('Look', function()
                g_game.look(mapPos, topItem:getId(), tile:getTopItemStackPos())
            end)

            if topItem:isUseable() then
                menu:addOption('Use', function()
                    g_game.use(mapPos, topItem:getId(), tile:getTopItemStackPos())
                end)
            end

            if topItem:isMultiUse() then
                menu:addOption('Use with...', function()
                    GameInterface.startUseWith(topItem, mapPos)
                end)
            end

            if topItem:isContainer() then
                menu:addOption('Open', function()
                    g_game.openContainer(mapPos, topItem:getId(), tile:getTopItemStackPos())
                end)
            end

            if topItem:isRotateable() then
                menu:addOption('Rotate', function()
                    g_game.rotate(mapPos, topItem:getId(), tile:getTopItemStackPos())
                end)
            end
        end
    end

    menu:addSeparator()
    menu:addOption('Copy Position', function()
        local posStr = string.format('%d, %d, %d', mapPos.x, mapPos.y, mapPos.z)
        g_window.setClipboardText(posStr)
    end)

    menu:display(screenPos)
    return true
end

function GameInterface.startUseWith(item, position)
    useWithItem = item
    useWithPosition = position
    g_mouse.setCursor('crosshair')
    GameInterface.addStatusMessage('Select target to use with...')
end

function GameInterface.completeUseWith(targetPos)
    if not useWithItem or not useWithPosition then
        GameInterface.cancelUseWith()
        return
    end

    local targetTile = g_map.getTile(targetPos)
    if targetTile then
        local topThing = targetTile:getTopThing()
        if topThing then
            g_game.useWith(useWithPosition, useWithItem:getId(), 0,
                          targetPos, topThing:getId(), 0)
        end
    end

    GameInterface.cancelUseWith()
end

function GameInterface.cancelUseWith()
    useWithItem = nil
    useWithPosition = nil
    g_mouse.setCursor('default')
end

function GameInterface.updateMousePosition(mapPos)
    -- Could update status bar with position info
end

function GameInterface.addStatusMessage(text)
    local statusBar = gameRootPanel:getChildById('statusBar')
    if statusBar then
        statusBar:setText(text)
    end
end

function GameInterface.getRootPanel()
    return gameRootPanel
end

function GameInterface.getMapPanel()
    return gameMapPanel
end

function GameInterface.getLeftPanel()
    return gameLeftPanel
end

function GameInterface.getRightPanel()
    return gameRightPanel
end

function GameInterface.getBottomPanel()
    return gameBottomPanel
end
