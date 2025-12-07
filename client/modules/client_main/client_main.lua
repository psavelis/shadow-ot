-- Shadow OT Client Main Module
-- Handles the main menu and login functionality

CLIENT_VERSION = 1285
CLIENT_NAME = "Shadow OT"

-- Server configuration
SERVER_HOST = "localhost"
SERVER_PORT = 7171

-- Module initialization
function init()
    print("[client_main] Initializing...")
    
    -- Connect keyboard shortcuts
    connect(g_app, {
        onExit = function() print("Exiting Shadow OT...") end
    })
    
    -- Show the main menu
    showMainMenu()
    
    print("[client_main] Initialized successfully")
end

-- Module termination
function terminate()
    print("[client_main] Terminating...")
    disconnect(g_app, {
        onExit = function() end
    })
end

-- Show the main menu
function showMainMenu()
    print("[client_main] Loading main menu...")
    
    -- Create main window
    local mainWindow = g_ui.displayUI('client_main.otui')
    if mainWindow then
        print("[client_main] Main menu loaded")
    else
        print("[client_main] Warning: Could not load main menu UI")
    end
end

-- Handle login button click
function onLoginClick()
    local accountField = mainMenu:getChildById('accountNameField')
    local passwordField = mainMenu:getChildById('passwordField')
    
    if accountField and passwordField then
        local account = accountField:getText()
        local password = passwordField:getText()
        
        if #account > 0 and #password > 0 then
            doLogin(account, password)
        else
            displayErrorBox("Login Error", "Please enter your account name and password.")
        end
    end
end

-- Perform the login
function doLogin(account, password)
    print("[client_main] Logging in as: " .. account)
    
    -- Connect to server
    local protocolLogin = ProtocolLogin.create()
    protocolLogin.onLoginError = onLoginError
    protocolLogin.onCharacterList = onCharacterList
    protocolLogin:login(SERVER_HOST, SERVER_PORT, account, password)
end

-- Handle login error
function onLoginError(message)
    displayErrorBox("Login Error", message or "Unknown error occurred.")
end

-- Handle character list received
function onCharacterList(characters, account, premiumDays)
    print("[client_main] Received character list: " .. #characters .. " characters")
    showCharacterList(characters, account, premiumDays)
end

-- Show character list
function showCharacterList(characters, account, premiumDays)
    local window = g_ui.displayUI('characterlist.otui')
    if window then
        local list = window:getChildById('characterList')
        for _, char in ipairs(characters) do
            local label = g_ui.createWidget('CharacterEntry', list)
            label:setText(char.name .. " - " .. char.world)
            label.character = char
        end
    end
end

-- Create a new account (redirects to web)
function onCreateAccountClick()
    g_platform.openUrl("http://localhost:3000/register")
end

-- Open settings window
function onSettingsClick()
    print("[client_main] Opening settings...")
    -- TODO: Implement settings window
end

-- Exit the game
function onExitClick()
    g_app.exit()
end


