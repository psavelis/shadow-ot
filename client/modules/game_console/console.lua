-- Shadow OT Console Module
-- Handles chat display and input

Console = {}

local consoleWindow = nil
local consoleBuffer = nil
local consoleInput = nil
local channelTabs = nil
local activeChannel = nil
local channels = {}

-- Message types
local MESSAGE_TYPES = {
    Say = 1,
    Whisper = 2,
    Yell = 3,
    Private = 4,
    Channel = 5,
    PrivateFrom = 6,
    PrivateTo = 7,
    NPC = 14,
    NPCFrom = 15,
    Broadcast = 16,
    GamemasterBroadcast = 17,
    GamemasterChannel = 18,
    GamemasterPrivateFrom = 19
}

-- Default channels
local DEFAULT_CHANNEL = 0
local LOCAL_CHANNEL = 0xFFFF

function Console.init()
    connect(g_game, {
        onGameStart = Console.onGameStart,
        onGameEnd = Console.onGameEnd,
        onTalk = Console.onTalk,
        onChannelList = Console.onChannelList,
        onOpenChannel = Console.onOpenChannel,
        onCloseChannel = Console.onCloseChannel,
        onOpenPrivateChannel = Console.onOpenPrivateChannel
    })

    consoleWindow = g_ui.displayUI('console')
    consoleBuffer = consoleWindow:getChildById('consoleBuffer')
    consoleInput = consoleWindow:getChildById('consoleInput')
    channelTabs = consoleWindow:getChildById('channelTabs')

    -- Input handling
    consoleInput.onTextChange = Console.onInputTextChange
    consoleInput.onKeyPress = Console.onInputKeyPress

    -- Create default local channel
    Console.createChannel(LOCAL_CHANNEL, 'Local Chat', true)

    consoleWindow:hide()
end

function Console.terminate()
    disconnect(g_game, {
        onGameStart = Console.onGameStart,
        onGameEnd = Console.onGameEnd,
        onTalk = Console.onTalk,
        onChannelList = Console.onChannelList,
        onOpenChannel = Console.onOpenChannel,
        onCloseChannel = Console.onCloseChannel,
        onOpenPrivateChannel = Console.onOpenPrivateChannel
    })

    if consoleWindow then
        consoleWindow:destroy()
        consoleWindow = nil
    end
end

function Console.onGameStart()
    consoleWindow:show()
    Console.selectChannel(LOCAL_CHANNEL)
end

function Console.onGameEnd()
    consoleWindow:hide()
    Console.clear()
end

function Console.createChannel(channelId, name, isLocal)
    if channels[channelId] then return end

    local channel = {
        id = channelId,
        name = name,
        messages = {},
        isLocal = isLocal or false,
        isPrivate = false
    }

    channels[channelId] = channel

    -- Create tab
    local tab = g_ui.createWidget('ChannelTab', channelTabs)
    tab.channelId = channelId
    tab:getChildById('name'):setText(name)

    tab.onClick = function()
        Console.selectChannel(channelId)
    end

    if not isLocal then
        local closeBtn = tab:getChildById('closeButton')
        closeBtn.onClick = function()
            Console.closeChannel(channelId)
        end
    end

    return channel
end

function Console.selectChannel(channelId)
    activeChannel = channels[channelId]
    if not activeChannel then return end

    -- Update tab highlighting
    for _, tab in pairs(channelTabs:getChildren()) do
        if tab.channelId == channelId then
            tab:setBackgroundColor('#3d5a80')
        else
            tab:setBackgroundColor('#2d2d2d')
        end
    end

    -- Display messages
    Console.refreshBuffer()
end

function Console.closeChannel(channelId)
    local channel = channels[channelId]
    if not channel or channel.isLocal then return end

    -- Remove tab
    for _, tab in pairs(channelTabs:getChildren()) do
        if tab.channelId == channelId then
            tab:destroy()
            break
        end
    end

    channels[channelId] = nil

    -- If closing active channel, switch to local
    if activeChannel and activeChannel.id == channelId then
        Console.selectChannel(LOCAL_CHANNEL)
    end

    -- Tell server to close
    if not channel.isPrivate then
        g_game.closeChannel(channelId)
    end
end

function Console.refreshBuffer()
    consoleBuffer:destroyChildren()

    if not activeChannel then return end

    for _, msg in ipairs(activeChannel.messages) do
        Console.addMessageWidget(msg)
    end

    consoleBuffer:scrollToBottom()
end

function Console.addMessageWidget(msg)
    local widget = g_ui.createWidget('ConsoleMessage', consoleBuffer)

    -- Time
    widget:getChildById('time'):setText(msg.time)

    -- Sender
    local senderWidget = widget:getChildById('sender')
    if msg.sender and #msg.sender > 0 then
        senderWidget:setText(msg.sender .. ': ')
        senderWidget:setColor(Console.getSenderColor(msg.type))

        senderWidget.onClick = function()
            Console.openPrivateChannel(msg.sender)
        end
    else
        senderWidget:setText('')
    end

    -- Message
    local textWidget = widget:getChildById('text')
    textWidget:setText(msg.text)
    textWidget:setColor(Console.getMessageColor(msg.type))
end

function Console.onTalk(name, level, type, text, channelId, pos)
    local targetChannel = channels[channelId] or channels[LOCAL_CHANNEL]
    if not targetChannel then return end

    local msg = {
        sender = name,
        level = level,
        type = type,
        text = text,
        time = os.date('%H:%M'),
        position = pos
    }

    table.insert(targetChannel.messages, msg)

    -- Limit message history
    while #targetChannel.messages > 1000 do
        table.remove(targetChannel.messages, 1)
    end

    -- Update display if active
    if activeChannel and activeChannel.id == targetChannel.id then
        Console.addMessageWidget(msg)
        consoleBuffer:scrollToBottom()
    end
end

function Console.onChannelList(channelList)
    -- Show channel selection dialog
    local dialog = g_ui.displayUI('channellist')
    local list = dialog:getChildById('channelList')

    for _, channel in ipairs(channelList) do
        local widget = g_ui.createWidget('ChannelListEntry', list)
        widget:getChildById('name'):setText(channel.name)
        widget.channelId = channel.id

        widget.onDoubleClick = function()
            g_game.openChannel(channel.id)
            dialog:destroy()
        end
    end

    dialog:getChildById('openButton').onClick = function()
        local selected = list:getFocusedChild()
        if selected then
            g_game.openChannel(selected.channelId)
        end
        dialog:destroy()
    end

    dialog:getChildById('cancelButton').onClick = function()
        dialog:destroy()
    end
end

function Console.onOpenChannel(channelId, name)
    Console.createChannel(channelId, name, false)
    Console.selectChannel(channelId)
end

function Console.onCloseChannel(channelId)
    Console.closeChannel(channelId)
end

function Console.onOpenPrivateChannel(name)
    Console.openPrivateChannel(name)
end

function Console.openPrivateChannel(name)
    -- Check if already open
    for id, channel in pairs(channels) do
        if channel.isPrivate and channel.name == name then
            Console.selectChannel(id)
            return
        end
    end

    -- Create private channel with unique ID
    local channelId = #channels + 1000
    local channel = Console.createChannel(channelId, name, false)
    channel.isPrivate = true

    Console.selectChannel(channelId)
end

function Console.onInputKeyPress(input, keyCode, keyText)
    if keyCode == 13 then -- Enter
        Console.sendMessage()
        return true
    end
    return false
end

function Console.onInputTextChange(input, text)
    -- Could add auto-complete here
end

function Console.sendMessage()
    local text = consoleInput:getText()
    if #text == 0 then return end

    consoleInput:setText('')

    if not activeChannel then return end

    if activeChannel.isLocal then
        -- Parse for commands or default to say
        if text:sub(1, 1) == '#' then
            -- Yell
            g_game.yell(text:sub(2))
        elseif text:sub(1, 1) == '$' then
            -- Whisper
            g_game.whisper(text:sub(2))
        elseif text:sub(1, 1) == '*' then
            -- Private message: *name* message
            local endName = text:find('*', 2)
            if endName then
                local name = text:sub(2, endName - 1)
                local msg = text:sub(endName + 1):match('^%s*(.+)')
                if msg then
                    g_game.privateMessage(name, msg)
                end
            end
        else
            g_game.say(text)
        end
    elseif activeChannel.isPrivate then
        g_game.privateMessage(activeChannel.name, text)
    else
        g_game.channelMessage(activeChannel.id, text)
    end
end

function Console.getSenderColor(type)
    if type == MESSAGE_TYPES.Say then return '#ffff00'
    elseif type == MESSAGE_TYPES.Whisper then return '#ffff00'
    elseif type == MESSAGE_TYPES.Yell then return '#ffff00'
    elseif type == MESSAGE_TYPES.Private then return '#00bfff'
    elseif type == MESSAGE_TYPES.PrivateFrom then return '#00bfff'
    elseif type == MESSAGE_TYPES.PrivateTo then return '#00bfff'
    elseif type == MESSAGE_TYPES.NPC then return '#00ff7f'
    elseif type == MESSAGE_TYPES.Broadcast then return '#f55'
    elseif type == MESSAGE_TYPES.GamemasterBroadcast then return '#f55'
    else return '#ffffff'
    end
end

function Console.getMessageColor(type)
    if type == MESSAGE_TYPES.Say then return '#ffff00'
    elseif type == MESSAGE_TYPES.Whisper then return '#ffff00'
    elseif type == MESSAGE_TYPES.Yell then return '#ffff00'
    elseif type == MESSAGE_TYPES.Private then return '#00bfff'
    elseif type == MESSAGE_TYPES.PrivateFrom then return '#00bfff'
    elseif type == MESSAGE_TYPES.PrivateTo then return '#00bfff'
    elseif type == MESSAGE_TYPES.NPC then return '#00ff7f'
    elseif type == MESSAGE_TYPES.Broadcast then return '#f55'
    elseif type == MESSAGE_TYPES.GamemasterBroadcast then return '#f55'
    else return '#ffffff'
    end
end

function Console.clear()
    for id, channel in pairs(channels) do
        channel.messages = {}
    end
    Console.refreshBuffer()
end

function Console.addMessage(text, type)
    local msg = {
        sender = '',
        type = type or 0,
        text = text,
        time = os.date('%H:%M')
    }

    local channel = channels[LOCAL_CHANNEL]
    if channel then
        table.insert(channel.messages, msg)
        if activeChannel and activeChannel.id == LOCAL_CHANNEL then
            Console.addMessageWidget(msg)
            consoleBuffer:scrollToBottom()
        end
    end
end

function Console.toggle()
    if consoleWindow:isVisible() then
        consoleWindow:hide()
    else
        consoleWindow:show()
        consoleWindow:raise()
    end
end
