-- Shadow OT Inventory Module
-- Handles equipment slots and inventory display

Inventory = {}

local inventoryWindow = nil
local slots = {}
local player = nil

-- Equipment slot IDs
local SLOT_HEAD = 1
local SLOT_NECKLACE = 2
local SLOT_BACKPACK = 3
local SLOT_ARMOR = 4
local SLOT_RIGHT_HAND = 5
local SLOT_LEFT_HAND = 6
local SLOT_LEGS = 7
local SLOT_FEET = 8
local SLOT_RING = 9
local SLOT_AMMO = 10

local slotNames = {
    [SLOT_HEAD] = 'head',
    [SLOT_NECKLACE] = 'necklace',
    [SLOT_BACKPACK] = 'backpack',
    [SLOT_ARMOR] = 'armor',
    [SLOT_RIGHT_HAND] = 'rightHand',
    [SLOT_LEFT_HAND] = 'leftHand',
    [SLOT_LEGS] = 'legs',
    [SLOT_FEET] = 'feet',
    [SLOT_RING] = 'ring',
    [SLOT_AMMO] = 'ammo'
}

function Inventory.init()
    connect(g_game, {
        onGameStart = Inventory.onGameStart,
        onGameEnd = Inventory.onGameEnd,
        onInventoryChange = Inventory.onInventoryChange
    })

    inventoryWindow = g_ui.displayUI('inventory')

    -- Get slot widgets
    for slotId, slotName in pairs(slotNames) do
        slots[slotId] = inventoryWindow:getChildById('slot' .. slotName:gsub("^%l", string.upper))
        Inventory.setupSlot(slots[slotId], slotId)
    end

    inventoryWindow:hide()
end

function Inventory.terminate()
    disconnect(g_game, {
        onGameStart = Inventory.onGameStart,
        onGameEnd = Inventory.onGameEnd,
        onInventoryChange = Inventory.onInventoryChange
    })

    if inventoryWindow then
        inventoryWindow:destroy()
        inventoryWindow = nil
    end
end

function Inventory.setupSlot(slotWidget, slotId)
    slotWidget.slotId = slotId

    -- Item display
    local itemWidget = slotWidget:getChildById('item')
    itemWidget:setVirtual(true)
    itemWidget:setItemId(0)

    -- Drop handling
    slotWidget.onDragEnter = function(self, item)
        if item and item:getItem() then
            self:setBorderColor('#00ff00')
            return true
        end
        return false
    end

    slotWidget.onDragLeave = function(self)
        self:setBorderColor('#444444')
    end

    slotWidget.onDrop = function(self, item, pos)
        if item and item:getItem() then
            local fromPos = item:getPosition()
            local toPos = {x = 0xFFFF, y = slotId, z = 0}
            g_game.move(fromPos, item:getId(), item:getStackPos(), toPos, item:getCount())
            return true
        end
        return false
    end

    -- Click handling
    slotWidget.onClick = function(self)
        local itemWidget = self:getChildById('item')
        local item = itemWidget:getItem()
        if item then
            g_game.use(Position.new(0xFFFF, slotId, 0), item:getId(), 0)
        end
    end

    slotWidget.onMouseRelease = function(self, mousePos, mouseButton)
        if mouseButton == 2 then
            local itemWidget = self:getChildById('item')
            local item = itemWidget:getItem()
            if item then
                Inventory.showItemMenu(item, slotId, mousePos)
            end
        end
    end
end

function Inventory.onGameStart()
    player = g_game.getLocalPlayer()
    inventoryWindow:show()
    Inventory.refresh()
end

function Inventory.onGameEnd()
    player = nil
    inventoryWindow:hide()
    Inventory.clear()
end

function Inventory.onInventoryChange(player, slot, item, oldItem)
    if slots[slot] then
        local itemWidget = slots[slot]:getChildById('item')
        if item then
            itemWidget:setItemId(item:getId())
            itemWidget:setItemCount(item:getCount())
        else
            itemWidget:setItemId(0)
        end
    end
end

function Inventory.refresh()
    if not player then return end

    for slotId, slotWidget in pairs(slots) do
        local item = player:getInventoryItem(slotId)
        local itemWidget = slotWidget:getChildById('item')
        if item then
            itemWidget:setItemId(item:getId())
            itemWidget:setItemCount(item:getCount())
        else
            itemWidget:setItemId(0)
        end
    end
end

function Inventory.clear()
    for _, slotWidget in pairs(slots) do
        local itemWidget = slotWidget:getChildById('item')
        itemWidget:setItemId(0)
    end
end

function Inventory.showItemMenu(item, slotId, mousePos)
    local menu = g_ui.createWidget('PopupMenu')

    menu:addOption('Look', function()
        g_game.look(Position.new(0xFFFF, slotId, 0), item:getId(), 0)
    end)

    if item:isUseable() then
        menu:addOption('Use', function()
            g_game.use(Position.new(0xFFFF, slotId, 0), item:getId(), 0)
        end)
    end

    if item:isMultiUse() then
        menu:addOption('Use with...', function()
            modules.game_interface.startUseWith(item, Position.new(0xFFFF, slotId, 0))
        end)
    end

    menu:addSeparator()

    if item:isMoveable() then
        menu:addOption('Move', function()
            -- Enable drag mode
        end)
    end

    if item:isContainer() then
        menu:addOption('Open', function()
            g_game.openContainer(Position.new(0xFFFF, slotId, 0), item:getId(), 0)
        end)
    end

    menu:display(mousePos)
end

function Inventory.toggle()
    if inventoryWindow:isVisible() then
        inventoryWindow:hide()
    else
        inventoryWindow:show()
        inventoryWindow:raise()
    end
end

function Inventory.show()
    inventoryWindow:show()
    inventoryWindow:raise()
end

function Inventory.hide()
    inventoryWindow:hide()
end

function Inventory.getWindow()
    return inventoryWindow
end
