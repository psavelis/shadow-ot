-- Shadow OT Realm Selection Module
-- Handles realm listing, selection, and themed UI

Realms = {}

local realmWindow = nil
local realmList = nil
local selectedRealm = nil
local refreshButton = nil
local connectButton = nil

-- Realm type icons
local realmTypeIcons = {
    dark = '/images/realms/dark_icon',
    mythic = '/images/realms/mythic_icon',
    pvp = '/images/realms/pvp_icon',
    classic = '/images/realms/classic_icon'
}

-- Realm type colors
local realmTypeColors = {
    dark = '#9b59b6',
    mythic = '#3498db',
    pvp = '#e74c3c',
    classic = '#c0a060'
}

function Realms.init()
    connect(g_game, {
        onGameStart = Realms.hide,
        onGameEnd = Realms.show
    })

    realmWindow = g_ui.displayUI('realms')
    realmList = realmWindow:getChildById('realmList')
    refreshButton = realmWindow:getChildById('refreshButton')
    connectButton = realmWindow:getChildById('connectButton')

    refreshButton.onClick = Realms.refresh
    connectButton.onClick = Realms.connect

    -- Initial load
    Realms.refresh()
end

function Realms.terminate()
    disconnect(g_game, {
        onGameStart = Realms.hide,
        onGameEnd = Realms.show
    })

    if realmWindow then
        realmWindow:destroy()
        realmWindow = nil
    end
end

function Realms.show()
    if realmWindow then
        realmWindow:show()
        realmWindow:raise()
        realmWindow:focus()
    end
end

function Realms.hide()
    if realmWindow then
        realmWindow:hide()
    end
end

function Realms.refresh()
    if not realmList then return end

    realmList:destroyChildren()
    refreshButton:setEnabled(false)
    refreshButton:setText('Loading...')

    -- Request realms from server/API
    g_http.get('https://api.shadow-ot.com/v1/realms', function(data, err)
        refreshButton:setEnabled(true)
        refreshButton:setText('Refresh')

        if err then
            Realms.showError('Failed to load realms: ' .. err)
            return
        end

        local realms = json.decode(data)
        Realms.populateList(realms)
    end)
end

function Realms.populateList(realms)
    if not realmList then return end

    realmList:destroyChildren()

    for _, realm in ipairs(realms) do
        local widget = g_ui.createWidget('RealmEntry', realmList)
        Realms.setupRealmEntry(widget, realm)
    end
end

function Realms.setupRealmEntry(widget, realm)
    -- Set realm data
    widget.realmData = realm

    -- Name and description
    widget:getChildById('name'):setText(realm.name)
    widget:getChildById('description'):setText(realm.description)

    -- Type badge
    local typeBadge = widget:getChildById('typeBadge')
    typeBadge:setText(Realms.getTypeDisplayName(realm.type))
    typeBadge:setColor(realmTypeColors[realm.type] or '#888888')

    -- Player count
    local playerCount = widget:getChildById('playerCount')
    playerCount:setText(string.format('%d/%d', realm.playersOnline, realm.maxPlayers))

    -- Load bar
    local loadBar = widget:getChildById('loadBar')
    local loadPercent = realm.playersOnline / realm.maxPlayers * 100
    loadBar:setPercent(loadPercent)
    loadBar:setBackgroundColor(Realms.getLoadColor(loadPercent))

    -- Status indicator
    local statusDot = widget:getChildById('statusDot')
    statusDot:setBackgroundColor(Realms.getStatusColor(realm.status))

    -- Rates
    widget:getChildById('expRate'):setText(string.format('%.1fx EXP', realm.expRate))
    widget:getChildById('lootRate'):setText(string.format('%.1fx Loot', realm.lootRate))

    -- PvP type
    widget:getChildById('pvpType'):setText(Realms.getPvPDisplayName(realm.pvpType))

    -- Premium indicator
    local premiumBadge = widget:getChildById('premiumBadge')
    premiumBadge:setVisible(realm.isPremium)

    -- Click handler
    widget.onClick = function()
        Realms.selectRealm(widget)
    end

    -- Double-click to connect
    widget.onDoubleClick = function()
        Realms.selectRealm(widget)
        Realms.connect()
    end
end

function Realms.selectRealm(widget)
    -- Deselect previous
    if selectedRealm then
        selectedRealm:setBackgroundColor('#2d2d2d')
    end

    -- Select new
    selectedRealm = widget
    widget:setBackgroundColor('#3d5a80')
    connectButton:setEnabled(true)

    -- Update theme preview
    Realms.updateThemePreview(widget.realmData)
end

function Realms.updateThemePreview(realm)
    local preview = realmWindow:getChildById('themePreview')
    if not preview then return end

    local colors = g_realms.getThemeColors(realm.type)
    preview:setBackgroundColor(colors.background)
    preview:getChildById('previewText'):setColor(colors.text)
    preview:getChildById('previewAccent'):setBackgroundColor(colors.accent)
end

function Realms.connect()
    if not selectedRealm or not selectedRealm.realmData then
        return
    end

    local realm = selectedRealm.realmData

    if realm.status ~= 'online' then
        Realms.showError('This realm is currently offline')
        return
    end

    if realm.playersOnline >= realm.maxPlayers then
        Realms.showError('This realm is full')
        return
    end

    connectButton:setEnabled(false)
    connectButton:setText('Connecting...')

    -- Store selected realm
    g_settings.set('lastRealm', realm.id)

    -- Connect to realm
    g_game.loginWorld(
        g_settings.get('account'),
        g_settings.get('password'),
        realm.ip,
        realm.port,
        realm.name,
        nil  -- character select will follow
    )
end

function Realms.getTypeDisplayName(type)
    local names = {
        dark = 'Dark Fantasy',
        mythic = 'Mythic',
        pvp = 'PvP Arena',
        classic = 'Classic'
    }
    return names[type] or 'Unknown'
end

function Realms.getPvPDisplayName(pvpType)
    local names = {
        open = 'Open PvP',
        optional = 'Optional PvP',
        hardcore = 'Hardcore',
        nonpvp = 'Non-PvP'
    }
    return names[pvpType] or 'Unknown'
end

function Realms.getStatusColor(status)
    local colors = {
        online = '#2ecc71',
        maintenance = '#f39c12',
        offline = '#e74c3c',
        full = '#9b59b6'
    }
    return colors[status] or '#888888'
end

function Realms.getLoadColor(percent)
    if percent < 50 then
        return '#2ecc71'  -- Green
    elseif percent < 80 then
        return '#f39c12'  -- Orange
    else
        return '#e74c3c'  -- Red
    end
end

function Realms.showError(message)
    displayErrorBox('Realm Error', message)
end

-- Filter functions
function Realms.filterByType(type)
    for _, widget in ipairs(realmList:getChildren()) do
        if type == 'all' or widget.realmData.type == type then
            widget:setVisible(true)
        else
            widget:setVisible(false)
        end
    end
end

function Realms.filterByPvP(pvpType)
    for _, widget in ipairs(realmList:getChildren()) do
        if pvpType == 'all' or widget.realmData.pvpType == pvpType then
            widget:setVisible(true)
        else
            widget:setVisible(false)
        end
    end
end

function Realms.sortByPlayers(descending)
    local children = realmList:getChildren()
    table.sort(children, function(a, b)
        if descending then
            return a.realmData.playersOnline > b.realmData.playersOnline
        else
            return a.realmData.playersOnline < b.realmData.playersOnline
        end
    end)

    realmList:destroyChildren()
    for _, widget in ipairs(children) do
        realmList:addChild(widget)
    end
end

function Realms.sortByName()
    local children = realmList:getChildren()
    table.sort(children, function(a, b)
        return a.realmData.name < b.realmData.name
    end)

    realmList:destroyChildren()
    for _, widget in ipairs(children) do
        realmList:addChild(widget)
    end
end
