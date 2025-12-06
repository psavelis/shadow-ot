-- Shadow OT Containers Module
-- Handles container windows and item management

Containers = {}

local containerWindows = {}
local draggingItem = nil

function Containers.init()
    connect(g_game, {
        onGameStart = Containers.onGameStart,
        onGameEnd = Containers.onGameEnd,
        onOpenContainer = Containers.onOpenContainer,
        onCloseContainer = Containers.onCloseContainer,
        onContainerAddItem = Containers.onContainerAddItem,
        onContainerUpdateItem = Containers.onContainerUpdateItem,
        onContainerRemoveItem = Containers.onContainerRemoveItem
    })
end

function Containers.terminate()
    disconnect(g_game, {
        onGameStart = Containers.onGameStart,
        onGameEnd = Containers.onGameEnd,
        onOpenContainer = Containers.onOpenContainer,
        onCloseContainer = Containers.onCloseContainer,
        onContainerAddItem = Containers.onContainerAddItem,
        onContainerUpdateItem = Containers.onContainerUpdateItem,
        onContainerRemoveItem = Containers.onContainerRemoveItem
    })

    Containers.closeAll()
end

function Containers.onGameStart()
    -- Nothing specific needed
end

function Containers.onGameEnd()
    Containers.closeAll()
end

function Containers.onOpenContainer(containerId, containerItemId, name, capacity, hasParent, items)
    -- Close existing window for this container
    Containers.close(containerId)

    -- Create new container window
    local window = g_ui.displayUI('container')
    window.containerId = containerId

    -- Set title
    window:setText(name)

    -- Set container icon
    local iconWidget = window:getChildById('containerIcon')
    iconWidget:setItemId(containerItemId)

    -- Setup up button
    local upButton = window:getChildById('upButton')
    if hasParent then
        upButton:setVisible(true)
        upButton.onClick = function()
            g_game.upContainer(containerId)
        end
    else
        upButton:setVisible(false)
    end

    -- Setup close button
    window.onClose = function()
        g_game.closeContainer(containerId)
    end

    -- Create item slots
    local itemPanel = window:getChildById('itemPanel')
    for i = 0, capacity - 1 do
        local slot = g_ui.createWidget('ContainerSlot', itemPanel)
        slot.slotIndex = i
        slot.containerId = containerId
        Containers.setupSlot(slot)
    end

    -- Add items
    for slot, item in pairs(items) do
        Containers.setSlotItem(containerId, slot, item)
    end

    containerWindows[containerId] = window

    -- Position window (cascade from previous)
    local offsetX = (containerId % 5) * 20
    local offsetY = (containerId % 5) * 20
    window:move(300 + offsetX, 200 + offsetY)
end

function Containers.onCloseContainer(containerId)
    Containers.close(containerId)
end

function Containers.onContainerAddItem(containerId, slot, item)
    Containers.setSlotItem(containerId, slot, item)
end

function Containers.onContainerUpdateItem(containerId, slot, item)
    Containers.setSlotItem(containerId, slot, item)
end

function Containers.onContainerRemoveItem(containerId, slot)
    Containers.setSlotItem(containerId, slot, nil)
end

function Containers.setupSlot(slot)
    local itemWidget = slot:getChildById('item')
    itemWidget:setVirtual(true)
    itemWidget:setItemId(0)

    -- Mouse events
    slot.onMousePress = function(self, pos, button)
        local item = itemWidget:getItem()
        if not item then return false end

        if button == 1 then
            -- Left click - use or start drag
            if g_keyboard.isCtrlPressed() then
                -- Use
                local containerPos = Position.new(0xFFFF, 0x40 + self.containerId, self.slotIndex)
                g_game.use(containerPos, item:getId(), self.slotIndex)
            else
                -- Start drag
                draggingItem = {
                    containerId = self.containerId,
                    slot = self.slotIndex,
                    item = item
                }
            end
            return true
        elseif button == 2 then
            -- Right click - context menu
            Containers.showItemMenu(self.containerId, self.slotIndex, item, pos)
            return true
        end
        return false
    end

    slot.onMouseRelease = function(self, pos, button)
        if button == 1 and draggingItem then
            -- Complete drag
            local targetSlot = Containers.getSlotAtPosition(pos)
            if targetSlot and targetSlot ~= slot then
                Containers.moveItem(draggingItem, targetSlot)
            end
            draggingItem = nil
            return true
        end
        return false
    end

    slot.onDragEnter = function(self)
        if draggingItem then
            self:setBorderColor('#00ff00')
            return true
        end
        return false
    end

    slot.onDragLeave = function(self)
        self:setBorderColor('#444444')
    end

    slot.onDrop = function(self, pos)
        if draggingItem then
            Containers.moveItem(draggingItem, self)
            draggingItem = nil
            return true
        end
        return false
    end
end

function Containers.setSlotItem(containerId, slot, item)
    local window = containerWindows[containerId]
    if not window then return end

    local itemPanel = window:getChildById('itemPanel')
    local slotWidget = itemPanel:getChildByIndex(slot + 1)
    if not slotWidget then return end

    local itemWidget = slotWidget:getChildById('item')
    if item then
        itemWidget:setItemId(item:getId())
        itemWidget:setItemCount(item:getCount())
    else
        itemWidget:setItemId(0)
    end
end

function Containers.getSlotAtPosition(pos)
    for _, window in pairs(containerWindows) do
        if window:containsPoint(pos) then
            local itemPanel = window:getChildById('itemPanel')
            for _, slot in pairs(itemPanel:getChildren()) do
                if slot:containsPoint(pos) then
                    return slot
                end
            end
        end
    end
    return nil
end

function Containers.moveItem(source, targetSlot)
    local fromPos = Position.new(0xFFFF, 0x40 + source.containerId, source.slot)
    local toPos = Position.new(0xFFFF, 0x40 + targetSlot.containerId, targetSlot.slotIndex)

    local count = source.item:getCount()
    if g_keyboard.isShiftPressed() and count > 1 then
        -- Show move amount dialog
        Containers.showMoveDialog(source.item, fromPos, toPos, count)
    else
        g_game.move(fromPos, source.item:getId(), source.slot, toPos, count)
    end
end

function Containers.showMoveDialog(item, fromPos, toPos, maxCount)
    local dialog = g_ui.displayUI('moveamount')
    local slider = dialog:getChildById('amountSlider')
    local input = dialog:getChildById('amountInput')

    slider:setRange(1, maxCount)
    slider:setValue(maxCount)
    input:setText(tostring(maxCount))

    slider.onValueChange = function(self, value)
        input:setText(tostring(value))
    end

    input.onTextChange = function(self, text)
        local value = tonumber(text) or 1
        value = math.max(1, math.min(maxCount, value))
        slider:setValue(value)
    end

    dialog:getChildById('okButton').onClick = function()
        local amount = slider:getValue()
        g_game.move(fromPos, item:getId(), fromPos.z, toPos, amount)
        dialog:destroy()
    end

    dialog:getChildById('cancelButton').onClick = function()
        dialog:destroy()
    end
end

function Containers.showItemMenu(containerId, slot, item, pos)
    local menu = g_ui.createWidget('PopupMenu')
    local containerPos = Position.new(0xFFFF, 0x40 + containerId, slot)

    menu:addOption('Look', function()
        g_game.look(containerPos, item:getId(), slot)
    end)

    if item:isUseable() then
        menu:addOption('Use', function()
            g_game.use(containerPos, item:getId(), slot)
        end)
    end

    if item:isMultiUse() then
        menu:addOption('Use with...', function()
            modules.game_interface.startUseWith(item, containerPos)
        end)
    end

    menu:addSeparator()

    if item:isContainer() then
        menu:addOption('Open', function()
            g_game.openContainer(containerPos, item:getId(), slot)
        end)
    end

    if item:isMoveable() then
        menu:addOption('Move', function()
            draggingItem = {
                containerId = containerId,
                slot = slot,
                item = item
            }
        end)
    end

    menu:addSeparator()

    if item:isReadable() then
        menu:addOption('Read', function()
            -- Open text window
        end)
    end

    menu:display(pos)
end

function Containers.close(containerId)
    local window = containerWindows[containerId]
    if window then
        window:destroy()
        containerWindows[containerId] = nil
    end
end

function Containers.closeAll()
    for containerId, window in pairs(containerWindows) do
        window:destroy()
    end
    containerWindows = {}
end

function Containers.getWindow(containerId)
    return containerWindows[containerId]
end

function Containers.getWindowCount()
    local count = 0
    for _ in pairs(containerWindows) do
        count = count + 1
    end
    return count
end
