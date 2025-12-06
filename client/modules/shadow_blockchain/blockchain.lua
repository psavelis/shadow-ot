-- Shadow OT Blockchain Module
-- Handles wallet connection, NFT display, and blockchain transactions

Blockchain = {}

local walletWindow = nil
local nftPanel = nil
local balancePanel = nil
local selectedNetwork = 'starknet'
local isConnected = false

-- Network configurations
local networks = {
    starknet = {
        name = 'Starknet',
        icon = '/images/blockchain/starknet',
        color = '#29296e',
        explorer = 'https://starkscan.co'
    },
    ethereum = {
        name = 'Ethereum',
        icon = '/images/blockchain/ethereum',
        color = '#627eea',
        explorer = 'https://etherscan.io'
    },
    polygon = {
        name = 'Polygon',
        icon = '/images/blockchain/polygon',
        color = '#8247e5',
        explorer = 'https://polygonscan.com'
    },
    bitcoin = {
        name = 'Bitcoin',
        icon = '/images/blockchain/bitcoin',
        color = '#f7931a',
        explorer = 'https://blockchain.info'
    },
    spark = {
        name = 'Spark',
        icon = '/images/blockchain/spark',
        color = '#00d4ff',
        explorer = 'https://spark-explorer.com'
    }
}

function Blockchain.init()
    connect(g_game, {
        onGameStart = Blockchain.onGameStart,
        onGameEnd = Blockchain.onGameEnd
    })

    walletWindow = g_ui.displayUI('blockchain')
    nftPanel = walletWindow:getChildById('nftPanel')
    balancePanel = walletWindow:getChildById('balancePanel')

    -- Setup network selector
    local networkSelector = walletWindow:getChildById('networkSelector')
    for network, config in pairs(networks) do
        networkSelector:addOption(config.name, network)
    end

    walletWindow:hide()
end

function Blockchain.terminate()
    disconnect(g_game, {
        onGameStart = Blockchain.onGameStart,
        onGameEnd = Blockchain.onGameEnd
    })

    if walletWindow then
        walletWindow:destroy()
        walletWindow = nil
    end
end

function Blockchain.onGameStart()
    -- Show wallet button in game UI
    Blockchain.createWalletButton()
end

function Blockchain.onGameEnd()
    Blockchain.disconnect()
end

function Blockchain.createWalletButton()
    local gameRootPanel = modules.game_interface.getRootPanel()
    if not gameRootPanel then return end

    local walletButton = g_ui.createWidget('WalletButton', gameRootPanel)
    walletButton:setId('walletButton')
    walletButton.onClick = Blockchain.toggle
end

function Blockchain.toggle()
    if walletWindow:isVisible() then
        walletWindow:hide()
    else
        walletWindow:show()
        walletWindow:raise()
        walletWindow:focus()
    end
end

function Blockchain.show()
    if walletWindow then
        walletWindow:show()
        walletWindow:raise()
    end
end

function Blockchain.hide()
    if walletWindow then
        walletWindow:hide()
    end
end

function Blockchain.connect()
    local connectButton = walletWindow:getChildById('connectButton')
    connectButton:setEnabled(false)
    connectButton:setText('Connecting...')

    g_wallet.connect(selectedNetwork, function(status, result)
        if status == 'connected' then
            isConnected = true
            Blockchain.onConnected(result)
        else
            isConnected = false
            Blockchain.showError('Connection failed: ' .. result)
        end

        connectButton:setEnabled(true)
        connectButton:setText(isConnected and 'Disconnect' or 'Connect Wallet')
    end)
end

function Blockchain.disconnect()
    g_wallet.disconnect()
    isConnected = false

    local connectButton = walletWindow:getChildById('connectButton')
    connectButton:setText('Connect Wallet')

    local addressLabel = walletWindow:getChildById('walletAddress')
    addressLabel:setText('Not connected')

    nftPanel:destroyChildren()
    balancePanel:destroyChildren()
end

function Blockchain.onConnected(address)
    local addressLabel = walletWindow:getChildById('walletAddress')
    addressLabel:setText(Blockchain.formatAddress(address))

    -- Load NFTs and balances
    Blockchain.loadNFTs()
    Blockchain.loadBalances()
end

function Blockchain.loadNFTs()
    if not isConnected then return end

    nftPanel:destroyChildren()

    local loadingLabel = g_ui.createWidget('Label', nftPanel)
    loadingLabel:setText('Loading NFTs...')

    g_wallet.loadNFTs(function(nfts)
        nftPanel:destroyChildren()

        if #nfts == 0 then
            local emptyLabel = g_ui.createWidget('Label', nftPanel)
            emptyLabel:setText('No NFTs found')
            return
        end

        for _, nft in ipairs(nfts) do
            local widget = g_ui.createWidget('NFTCard', nftPanel)
            Blockchain.setupNFTCard(widget, nft)
        end
    end)
end

function Blockchain.setupNFTCard(widget, nft)
    widget.nftData = nft

    -- NFT image
    local image = widget:getChildById('nftImage')
    if nft.imageUrl then
        image:setImageSource(nft.imageUrl)
    end

    -- NFT name
    widget:getChildById('nftName'):setText(nft.name)

    -- Rarity badge
    local rarityBadge = widget:getChildById('rarityBadge')
    rarityBadge:setText(nft.rarity)
    rarityBadge:setColor(Blockchain.getRarityColor(nft.rarity))

    -- Equipped indicator
    local equippedBadge = widget:getChildById('equippedBadge')
    equippedBadge:setVisible(nft.isEquipped)

    -- Actions
    widget:getChildById('equipButton').onClick = function()
        Blockchain.equipNFT(nft)
    end

    widget:getChildById('viewButton').onClick = function()
        Blockchain.viewNFTDetails(nft)
    end
end

function Blockchain.loadBalances()
    if not isConnected then return end

    balancePanel:destroyChildren()

    g_wallet.loadBalances(function(balances)
        balancePanel:destroyChildren()

        for _, balance in ipairs(balances) do
            local widget = g_ui.createWidget('BalanceEntry', balancePanel)
            widget:getChildById('symbol'):setText(balance.symbol)
            widget:getChildById('amount'):setText(balance.amount)
            widget:getChildById('usdValue'):setText(balance.usdValue)
        end
    end)
end

function Blockchain.equipNFT(nft)
    if nft.inGameItemId == 0 then
        Blockchain.showError('This NFT cannot be equipped')
        return
    end

    if nft.isEquipped then
        g_wallet.unequipNFT(nft.tokenId)
        nft.isEquipped = false
    else
        g_wallet.equipNFT(nft.tokenId)
        nft.isEquipped = true
    end

    -- Refresh NFT display
    Blockchain.loadNFTs()
end

function Blockchain.viewNFTDetails(nft)
    local detailsWindow = g_ui.displayUI('nft_details')

    detailsWindow:getChildById('nftName'):setText(nft.name)
    detailsWindow:getChildById('nftDescription'):setText(nft.description)
    detailsWindow:getChildById('nftImage'):setImageSource(nft.imageUrl)
    detailsWindow:getChildById('tokenId'):setText('Token ID: ' .. nft.tokenId)
    detailsWindow:getChildById('contract'):setText('Contract: ' .. Blockchain.formatAddress(nft.contractAddress))

    -- Attributes
    local attrPanel = detailsWindow:getChildById('attributesPanel')
    for key, value in pairs(nft.attributes) do
        local attrWidget = g_ui.createWidget('AttributeEntry', attrPanel)
        attrWidget:getChildById('attrName'):setText(key)
        attrWidget:getChildById('attrValue'):setText(value)
    end

    -- View on explorer button
    detailsWindow:getChildById('explorerButton').onClick = function()
        local network = networks[networkToString(nft.network)]
        local url = network.explorer .. '/token/' .. nft.contractAddress .. '/' .. nft.tokenId
        g_platform.openUrl(url)
    end
end

function Blockchain.formatAddress(address)
    if #address <= 13 then
        return address
    end
    return address:sub(1, 6) .. '...' .. address:sub(-4)
end

function Blockchain.getRarityColor(rarity)
    local colors = {
        common = '#888888',
        uncommon = '#2ecc71',
        rare = '#3498db',
        epic = '#9b59b6',
        legendary = '#f39c12'
    }
    return colors[rarity:lower()] or '#888888'
end

function Blockchain.showError(message)
    displayErrorBox('Wallet Error', message)
end

function Blockchain.setNetwork(network)
    selectedNetwork = network
    if isConnected then
        Blockchain.disconnect()
    end
end

-- Marketplace functions
function Blockchain.listForSale(nft, price)
    g_wallet.listNFTForSale(nft.tokenId, price, function(tx)
        if tx.status == 'pending' then
            displayInfoBox('Listing', 'Your NFT is being listed. Transaction: ' .. tx.hash)
        elseif tx.status == 'confirmed' then
            displayInfoBox('Listed', 'Your NFT has been listed for sale!')
        else
            Blockchain.showError('Listing failed')
        end
    end)
end

function Blockchain.buyNFT(contractAddress, tokenId)
    g_wallet.buyNFT(contractAddress, tokenId, function(tx)
        if tx.status == 'pending' then
            displayInfoBox('Purchase', 'Processing your purchase. Transaction: ' .. tx.hash)
        elseif tx.status == 'confirmed' then
            displayInfoBox('Purchased', 'NFT purchased successfully!')
            Blockchain.loadNFTs()
        else
            Blockchain.showError('Purchase failed')
        end
    end)
end

-- Wallet authentication for login
function Blockchain.signLogin(callback)
    local message = 'Sign this message to login to Shadow OT\nTimestamp: ' .. os.time()

    g_wallet.signMessage(message, function(signature)
        if signature and #signature > 0 then
            callback(true, signature, message)
        else
            callback(false, nil, nil)
        end
    end)
end
