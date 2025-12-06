-- Shadow OT Minimap Module
-- Displays the game world minimap

Minimap = {}

local minimapWindow = nil
local minimapWidget = nil
local zoomLevel = 2  -- 1-4
local floorLevel = 7
local centerPosition = nil
local followPlayer = true

-- Minimap colors (based on Tibia's minimap colors)
local colors = {
    [0] = {0, 0, 0},        -- Black (unknown/void)
    [12] = {0, 102, 0},      -- Dark green (grass)
    [24] = {0, 204, 0},      -- Green (more grass)
    [30] = {0, 255, 0},      -- Bright green
    [40] = {51, 0, 204},     -- Blue (water)
    [86] = {102, 102, 102},  -- Gray (stone)
    [114] = {153, 153, 153}, -- Light gray
    [129] = {128, 128, 0},   -- Olive
    [140] = {102, 51, 0},    -- Brown (dirt)
    [179] = {204, 0, 0},     -- Red (lava)
    [186] = {255, 204, 0},   -- Yellow (sand)
    [192] = {255, 102, 0},   -- Orange
    [207] = {255, 255, 255}, -- White (snow)
    [210] = {255, 255, 0},   -- Bright yellow
}

function Minimap.init()
    connect(g_game, {
        onGameStart = Minimap.onGameStart,
        onGameEnd = Minimap.onGameEnd
    })

    connect(LocalPlayer, {
        onPositionChange = Minimap.onPlayerPositionChange
    })

    minimapWindow = g_ui.displayUI('minimap')
    minimapWidget = minimapWindow:getChildById('minimapView')

    -- Setup controls
    local zoomInBtn = minimapWindow:getChildById('zoomIn')
    local zoomOutBtn = minimapWindow:getChildById('zoomOut')
    local floorUpBtn = minimapWindow:getChildById('floorUp')
    local floorDownBtn = minimapWindow:getChildById('floorDown')
    local centerBtn = minimapWindow:getChildById('centerOnPlayer')

    zoomInBtn.onClick = Minimap.zoomIn
    zoomOutBtn.onClick = Minimap.zoomOut
    floorUpBtn.onClick = Minimap.floorUp
    floorDownBtn.onClick = Minimap.floorDown
    centerBtn.onClick = Minimap.centerOnPlayer

    -- Mouse controls
    minimapWidget.onMouseWheel = function(self, pos, delta)
        if delta > 0 then
            Minimap.zoomIn()
        else
            Minimap.zoomOut()
        end
    end

    minimapWidget.onMousePress = function(self, pos, button)
        if button == 1 then
            followPlayer = false
            local worldPos = Minimap.screenToWorld(pos.x, pos.y)
            centerPosition = worldPos
            Minimap.updateView()
        elseif button == 2 then
            local worldPos = Minimap.screenToWorld(pos.x, pos.y)
            Minimap.showContextMenu(worldPos, pos)
        end
    end

    minimapWindow:hide()
end

function Minimap.terminate()
    disconnect(g_game, {
        onGameStart = Minimap.onGameStart,
        onGameEnd = Minimap.onGameEnd
    })

    disconnect(LocalPlayer, {
        onPositionChange = Minimap.onPlayerPositionChange
    })

    if minimapWindow then
        minimapWindow:destroy()
        minimapWindow = nil
    end
end

function Minimap.onGameStart()
    local player = g_game.getLocalPlayer()
    if player then
        centerPosition = player:getPosition()
        floorLevel = centerPosition.z
    end
    followPlayer = true
    minimapWindow:show()
    Minimap.updateView()
end

function Minimap.onGameEnd()
    minimapWindow:hide()
    centerPosition = nil
end

function Minimap.onPlayerPositionChange(player, newPos, oldPos)
    if followPlayer then
        centerPosition = newPos
        floorLevel = newPos.z
        Minimap.updateView()
    end
end

function Minimap.updateView()
    if not centerPosition or not minimapWidget then return end

    -- Update floor label
    local floorLabel = minimapWindow:getChildById('floorLabel')
    floorLabel:setText(string.format('Floor: %d', floorLevel))

    -- Update minimap rendering
    minimapWidget:setCenter(centerPosition.x, centerPosition.y, floorLevel)
    minimapWidget:setZoom(zoomLevel)

    -- Update zoom label
    local zoomLabel = minimapWindow:getChildById('zoomLabel')
    zoomLabel:setText(string.format('%dx', zoomLevel))
end

function Minimap.zoomIn()
    if zoomLevel < 4 then
        zoomLevel = zoomLevel + 1
        Minimap.updateView()
    end
end

function Minimap.zoomOut()
    if zoomLevel > 1 then
        zoomLevel = zoomLevel - 1
        Minimap.updateView()
    end
end

function Minimap.floorUp()
    if floorLevel > 0 then
        floorLevel = floorLevel - 1
        followPlayer = false
        Minimap.updateView()
    end
end

function Minimap.floorDown()
    if floorLevel < 15 then
        floorLevel = floorLevel + 1
        followPlayer = false
        Minimap.updateView()
    end
end

function Minimap.centerOnPlayer()
    local player = g_game.getLocalPlayer()
    if player then
        centerPosition = player:getPosition()
        floorLevel = centerPosition.z
        followPlayer = true
        Minimap.updateView()
    end
end

function Minimap.screenToWorld(screenX, screenY)
    local viewRect = minimapWidget:getRect()
    local tileSize = minimapWidget:getTileSize() * zoomLevel

    local offsetX = screenX - viewRect.x - viewRect.width / 2
    local offsetY = screenY - viewRect.y - viewRect.height / 2

    local worldX = centerPosition.x + math.floor(offsetX / tileSize)
    local worldY = centerPosition.y + math.floor(offsetY / tileSize)

    return {x = worldX, y = worldY, z = floorLevel}
end

function Minimap.worldToScreen(worldX, worldY, worldZ)
    if worldZ ~= floorLevel then return nil end

    local viewRect = minimapWidget:getRect()
    local tileSize = minimapWidget:getTileSize() * zoomLevel

    local offsetX = (worldX - centerPosition.x) * tileSize
    local offsetY = (worldY - centerPosition.y) * tileSize

    local screenX = viewRect.x + viewRect.width / 2 + offsetX
    local screenY = viewRect.y + viewRect.height / 2 + offsetY

    return {x = screenX, y = screenY}
end

function Minimap.showContextMenu(worldPos, screenPos)
    local menu = g_ui.createWidget('PopupMenu')

    menu:addOption('Mark Position', function()
        Minimap.addMark(worldPos)
    end)

    menu:addOption('Autowalk Here', function()
        local player = g_game.getLocalPlayer()
        if player then
            local path = g_map.findPath(player:getPosition(), worldPos)
            if #path > 0 then
                g_game.autoWalk(path)
            end
        end
    end)

    menu:addSeparator()

    menu:addOption('Copy Position', function()
        local posStr = string.format('%d, %d, %d', worldPos.x, worldPos.y, worldPos.z)
        g_window.setClipboardText(posStr)
    end)

    menu:display(screenPos)
end

function Minimap.addMark(position, description, icon)
    description = description or ''
    icon = icon or 0

    -- Store mark in minimap data
    g_minimap.addMark(position.x, position.y, position.z, icon, description)
    Minimap.updateView()
end

function Minimap.removeMark(position)
    g_minimap.removeMark(position.x, position.y, position.z)
    Minimap.updateView()
end

function Minimap.getMarks()
    return g_minimap.getMarks()
end

function Minimap.toggle()
    if minimapWindow:isVisible() then
        minimapWindow:hide()
    else
        minimapWindow:show()
        minimapWindow:raise()
    end
end

function Minimap.show()
    minimapWindow:show()
    minimapWindow:raise()
end

function Minimap.hide()
    minimapWindow:hide()
end

function Minimap.save()
    -- Save minimap data to file
    local data = g_minimap.serialize()
    g_resources.writeFile('/minimap.otmm', data)
end

function Minimap.load()
    -- Load minimap data from file
    local data = g_resources.readFile('/minimap.otmm')
    if data then
        g_minimap.deserialize(data)
    end
end
