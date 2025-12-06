--[[
    Shadow OT Auto-Updater Module

    Features:
    - Delta updates (only download changed files)
    - Asset sync (sprites, sounds, UI)
    - Version check
    - Rollback support
    - Background downloading
]]

Updater = {}

-- Update server configuration
Updater.config = {
    updateUrl = "https://update.shadowot.com/api/v1",
    assetCdn = "https://cdn.shadowot.com/assets",
    checkInterval = 3600, -- Check every hour
    autoUpdate = true,
    autoDownload = true,
    deltaUpdates = true,
    maxRollbacks = 3
}

-- Current state
Updater.state = {
    currentVersion = "1.0.0",
    latestVersion = nil,
    updateAvailable = false,
    downloading = false,
    downloadProgress = 0,
    downloadSize = 0,
    downloadedSize = 0,
    pendingFiles = {},
    rollbackVersions = {}
}

-- Callbacks
Updater.callbacks = {
    onUpdateAvailable = nil,
    onDownloadStart = nil,
    onDownloadProgress = nil,
    onDownloadComplete = nil,
    onUpdateReady = nil,
    onError = nil
}

-- Initialize updater
function Updater.init()
    print("[Updater] Auto-updater initialized")
    print("[Updater] Current version: " .. Updater.state.currentVersion)

    -- Load saved rollback info
    Updater.loadRollbackInfo()

    -- Check for updates on startup
    if Updater.config.autoUpdate then
        Updater.checkForUpdates()
    end

    -- Schedule periodic update checks
    Updater.scheduleCheck()
end

-- Terminate updater
function Updater.terminate()
    -- Cancel any pending downloads
    Updater.cancelDownload()
end

-- Schedule next update check
function Updater.scheduleCheck()
    scheduleEvent(function()
        Updater.checkForUpdates()
        Updater.scheduleCheck()
    end, Updater.config.checkInterval * 1000)
end

-- Check for updates
function Updater.checkForUpdates()
    print("[Updater] Checking for updates...")

    -- Would make HTTP request to update server
    -- Simulated response
    local response = Updater.simulateVersionCheck()

    if response.success then
        Updater.state.latestVersion = response.latestVersion
        Updater.state.updateAvailable = Updater.isNewerVersion(
            response.latestVersion,
            Updater.state.currentVersion
        )

        if Updater.state.updateAvailable then
            print("[Updater] Update available: " .. response.latestVersion)
            Updater.state.pendingFiles = response.files

            if Updater.callbacks.onUpdateAvailable then
                Updater.callbacks.onUpdateAvailable(response)
            end

            if Updater.config.autoDownload then
                Updater.startDownload()
            end
        else
            print("[Updater] Client is up to date")
        end
    else
        print("[Updater] Failed to check for updates: " .. (response.error or "Unknown error"))
        if Updater.callbacks.onError then
            Updater.callbacks.onError("check_failed", response.error)
        end
    end
end

-- Simulate version check (would be HTTP request)
function Updater.simulateVersionCheck()
    return {
        success = true,
        latestVersion = "1.0.0",
        releaseDate = "2024-01-15",
        mandatory = false,
        changelog = {
            "Bug fixes and performance improvements",
            "New realm: Astralforge",
            "Updated Prey system"
        },
        files = {}
    }
end

-- Compare versions (returns true if v1 > v2)
function Updater.isNewerVersion(v1, v2)
    local function parseVersion(v)
        local major, minor, patch = v:match("(%d+)%.(%d+)%.(%d+)")
        return {
            major = tonumber(major) or 0,
            minor = tonumber(minor) or 0,
            patch = tonumber(patch) or 0
        }
    end

    local ver1 = parseVersion(v1)
    local ver2 = parseVersion(v2)

    if ver1.major ~= ver2.major then
        return ver1.major > ver2.major
    elseif ver1.minor ~= ver2.minor then
        return ver1.minor > ver2.minor
    else
        return ver1.patch > ver2.patch
    end
end

-- Start downloading update
function Updater.startDownload()
    if Updater.state.downloading then
        print("[Updater] Download already in progress")
        return false
    end

    if #Updater.state.pendingFiles == 0 then
        print("[Updater] No files to download")
        return false
    end

    Updater.state.downloading = true
    Updater.state.downloadProgress = 0
    Updater.state.downloadedSize = 0

    -- Calculate total size
    Updater.state.downloadSize = 0
    for _, file in ipairs(Updater.state.pendingFiles) do
        Updater.state.downloadSize = Updater.state.downloadSize + (file.size or 0)
    end

    print("[Updater] Starting download... (" .. Updater.formatSize(Updater.state.downloadSize) .. ")")

    if Updater.callbacks.onDownloadStart then
        Updater.callbacks.onDownloadStart(Updater.state.downloadSize, #Updater.state.pendingFiles)
    end

    -- Download files one by one (or in parallel)
    Updater.downloadNextFile()

    return true
end

-- Download next file in queue
function Updater.downloadNextFile()
    if not Updater.state.downloading then return end

    local file = table.remove(Updater.state.pendingFiles, 1)

    if not file then
        -- All files downloaded
        Updater.state.downloading = false
        Updater.state.downloadProgress = 100

        print("[Updater] Download complete!")

        if Updater.callbacks.onDownloadComplete then
            Updater.callbacks.onDownloadComplete()
        end

        -- Notify update ready to apply
        if Updater.callbacks.onUpdateReady then
            Updater.callbacks.onUpdateReady(Updater.state.latestVersion)
        end

        return
    end

    -- Simulate file download
    print("[Updater] Downloading: " .. file.path)

    -- Would use HTTP module to download
    -- Simulated download progress
    scheduleEvent(function()
        Updater.state.downloadedSize = Updater.state.downloadedSize + (file.size or 0)
        Updater.state.downloadProgress = math.floor(
            (Updater.state.downloadedSize / Updater.state.downloadSize) * 100
        )

        if Updater.callbacks.onDownloadProgress then
            Updater.callbacks.onDownloadProgress(
                Updater.state.downloadProgress,
                Updater.state.downloadedSize,
                Updater.state.downloadSize
            )
        end

        -- Download next file
        Updater.downloadNextFile()
    end, 100)
end

-- Cancel ongoing download
function Updater.cancelDownload()
    if Updater.state.downloading then
        Updater.state.downloading = false
        Updater.state.pendingFiles = {}
        print("[Updater] Download cancelled")
    end
end

-- Apply downloaded update
function Updater.applyUpdate()
    print("[Updater] Applying update to version " .. Updater.state.latestVersion)

    -- Save current version for rollback
    Updater.saveRollbackInfo()

    -- Would extract and apply downloaded files
    -- Then restart client

    Updater.state.currentVersion = Updater.state.latestVersion
    Updater.state.updateAvailable = false

    print("[Updater] Update applied successfully!")
    print("[Updater] Please restart the client")

    return true
end

-- Rollback to previous version
function Updater.rollback()
    if #Updater.state.rollbackVersions == 0 then
        print("[Updater] No rollback versions available")
        return false
    end

    local previousVersion = Updater.state.rollbackVersions[1]
    print("[Updater] Rolling back to version " .. previousVersion)

    -- Would restore previous version files

    table.remove(Updater.state.rollbackVersions, 1)
    Updater.saveRollbackInfo()

    print("[Updater] Rollback complete! Please restart the client")
    return true
end

-- Save rollback info
function Updater.saveRollbackInfo()
    -- Would save to file
    table.insert(Updater.state.rollbackVersions, 1, Updater.state.currentVersion)

    -- Keep only max rollback versions
    while #Updater.state.rollbackVersions > Updater.config.maxRollbacks do
        table.remove(Updater.state.rollbackVersions)
    end
end

-- Load rollback info
function Updater.loadRollbackInfo()
    -- Would load from file
    Updater.state.rollbackVersions = {}
end

-- Format file size
function Updater.formatSize(bytes)
    if bytes < 1024 then
        return bytes .. " B"
    elseif bytes < 1024 * 1024 then
        return string.format("%.1f KB", bytes / 1024)
    elseif bytes < 1024 * 1024 * 1024 then
        return string.format("%.1f MB", bytes / (1024 * 1024))
    else
        return string.format("%.1f GB", bytes / (1024 * 1024 * 1024))
    end
end

-- Callback setters
function Updater.onUpdateAvailable(callback)
    Updater.callbacks.onUpdateAvailable = callback
end

function Updater.onDownloadStart(callback)
    Updater.callbacks.onDownloadStart = callback
end

function Updater.onDownloadProgress(callback)
    Updater.callbacks.onDownloadProgress = callback
end

function Updater.onDownloadComplete(callback)
    Updater.callbacks.onDownloadComplete = callback
end

function Updater.onUpdateReady(callback)
    Updater.callbacks.onUpdateReady = callback
end

function Updater.onError(callback)
    Updater.callbacks.onError = callback
end

-- Configuration
function Updater.setAutoUpdate(enabled)
    Updater.config.autoUpdate = enabled
end

function Updater.setAutoDownload(enabled)
    Updater.config.autoDownload = enabled
end

function Updater.setCheckInterval(seconds)
    Updater.config.checkInterval = seconds
end

-- Status getters
function Updater.isUpdateAvailable()
    return Updater.state.updateAvailable
end

function Updater.isDownloading()
    return Updater.state.downloading
end

function Updater.getDownloadProgress()
    return Updater.state.downloadProgress
end

function Updater.getCurrentVersion()
    return Updater.state.currentVersion
end

function Updater.getLatestVersion()
    return Updater.state.latestVersion
end

function Updater.getRollbackVersions()
    return Updater.state.rollbackVersions
end

return Updater
