#!/bin/bash
# Shadow OT - Complete Asset Download Script
# Downloads all required assets for a playable OT client
# 
# Usage: ./scripts/download-all-assets.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CLIENT_DATA="$ROOT_DIR/client/data"
SERVER_DATA="$ROOT_DIR/data"
ASSETS_DIR="$ROOT_DIR/assets"

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║           Shadow OT - Complete Asset Download                 ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Create directories
echo "[1/7] Creating directories..."
mkdir -p "$CLIENT_DATA/sprites"
mkdir -p "$CLIENT_DATA/things"
mkdir -p "$CLIENT_DATA/ui/images"
mkdir -p "$CLIENT_DATA/music"
mkdir -p "$CLIENT_DATA/sounds"
mkdir -p "$SERVER_DATA/maps"
mkdir -p "$SERVER_DATA/items"
mkdir -p "$ASSETS_DIR/sprites"
mkdir -p "$ASSETS_DIR/maps"

# ============================================================================
# STEP 1: Download OTClient assets from mehah/otclient releases
# ============================================================================
echo ""
echo "[2/7] Downloading OTClient assets (Tibia.spr, Tibia.dat)..."

OTCLIENT_RELEASE="https://github.com/mehah/otclient/releases/download/v1.0.1"
TEMP_DIR=$(mktemp -d)

cd "$TEMP_DIR"

# Download OTClient release for data files
if command -v curl &> /dev/null; then
    curl -L -o otclient.zip "$OTCLIENT_RELEASE/otclient-windows-x64.zip" || {
        echo "Failed to download OTClient, trying alternative..."
        # Alternative: TibiaMaps sprites
        curl -L -o Tibia.spr "https://raw.githubusercontent.com/AlanDevelworker/tibia-spr/main/Tibia.spr" || true
    }
elif command -v wget &> /dev/null; then
    wget -O otclient.zip "$OTCLIENT_RELEASE/otclient-windows-x64.zip" || true
fi

# Extract if zip exists
if [ -f "otclient.zip" ]; then
    unzip -q otclient.zip -d otclient_extract || true
    
    # Copy sprite data files
    find otclient_extract -name "*.dat" -exec cp {} "$CLIENT_DATA/things/" \; 2>/dev/null || true
    find otclient_extract -name "*.spr" -exec cp {} "$CLIENT_DATA/things/" \; 2>/dev/null || true
    find otclient_extract -name "*.otb" -exec cp {} "$CLIENT_DATA/things/" \; 2>/dev/null || true
    find otclient_extract -name "*.otfi" -exec cp {} "$CLIENT_DATA/things/" \; 2>/dev/null || true
    
    # Copy UI images
    find otclient_extract -path "*/images/*" -name "*.png" -exec cp {} "$CLIENT_DATA/ui/images/" \; 2>/dev/null || true
    
    # Copy fonts
    find otclient_extract -name "*.otfont" -exec cp {} "$CLIENT_DATA/fonts/" \; 2>/dev/null || true
    
    # Copy modules
    if [ -d "otclient_extract" ]; then
        find otclient_extract -type d -name "modules" -exec cp -r {}/* "$ROOT_DIR/client/modules/" \; 2>/dev/null || true
    fi
fi

# ============================================================================
# STEP 2: Download TibiaMaps data (sprites and map tiles)
# ============================================================================
echo ""
echo "[3/7] Downloading TibiaMaps sprites..."

# TibiaMaps has minimap sprites we can use
TIBIAMAPS_BASE="https://tibiamaps.github.io/tibia-map-data"

curl -L -o "$ASSETS_DIR/sprites/Tibia.pic" "$TIBIAMAPS_BASE/mapper-sprites/Tibia.pic" 2>/dev/null || true

# ============================================================================
# STEP 3: Download Canary server data (items.otb, monsters, etc.)
# ============================================================================
echo ""
echo "[4/7] Downloading server data from Canary..."

CANARY_BASE="https://raw.githubusercontent.com/opentibiabr/canary/main/data"

# Download items data
curl -L -o "$SERVER_DATA/items/items.xml" "$CANARY_BASE/items/items.xml" 2>/dev/null || true

# Download world map (OTBM) - this is the main map file
echo "Downloading world map (this may take a while)..."
curl -L -o "$SERVER_DATA/maps/world.otbm" "https://github.com/opentibiabr/canary/raw/main/data/world/world.otbm" 2>/dev/null || {
    # Alternative: smaller test map
    echo "Large map failed, trying smaller test map..."
    curl -L -o "$SERVER_DATA/maps/test.otbm" "https://github.com/otland/forgottenserver/raw/master/data/world/forgotten.otbm" 2>/dev/null || true
}

# Download spawn data
curl -L -o "$SERVER_DATA/maps/world-spawns.xml" "$CANARY_BASE/world/world-spawns.xml" 2>/dev/null || true

# Download house data  
curl -L -o "$SERVER_DATA/maps/world-houses.xml" "$CANARY_BASE/world/world-houses.xml" 2>/dev/null || true

# ============================================================================
# STEP 4: Create/Download UI sprites
# ============================================================================
echo ""
echo "[5/7] Creating UI sprite assets..."

# Create basic UI sprites if not downloaded
cd "$CLIENT_DATA/ui/images"

# Create placeholder sprites using ImageMagick if available, otherwise create minimal PNG
if command -v convert &> /dev/null; then
    # Button sprites
    convert -size 100x30 xc:'#4a4a4a' -draw "roundrectangle 2,2 97,27 5,5" button_normal.png 2>/dev/null || true
    convert -size 100x30 xc:'#5a5a5a' -draw "roundrectangle 2,2 97,27 5,5" button_hover.png 2>/dev/null || true
    convert -size 100x30 xc:'#3a3a3a' -draw "roundrectangle 2,2 97,27 5,5" button_pressed.png 2>/dev/null || true
    
    # Window sprite
    convert -size 400x300 xc:'#2a2a2a' -stroke '#4a4a4a' -strokewidth 2 -fill none -draw "rectangle 1,1 398,298" window.png 2>/dev/null || true
    
    # Scrollbar sprites
    convert -size 15x100 xc:'#3a3a3a' scrollbar_bg.png 2>/dev/null || true
    convert -size 15x30 xc:'#5a5a5a' scrollbar_handle.png 2>/dev/null || true
    
    # Health/mana bars
    convert -size 100x10 xc:'#aa0000' healthbar.png 2>/dev/null || true
    convert -size 100x10 xc:'#0000aa' manabar.png 2>/dev/null || true
    
    # Inventory slot
    convert -size 32x32 xc:'#333333' -stroke '#555555' -strokewidth 1 -fill none -draw "rectangle 0,0 31,31" inventory_slot.png 2>/dev/null || true
else
    echo "ImageMagick not found, creating minimal placeholder files..."
    # Create minimal 1x1 PNG placeholders (PNG header + minimal data)
    printf '\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x02\x00\x00\x00\x90wS\xde\x00\x00\x00\x0cIDATx\x9cc\xf8\x0f\x00\x00\x01\x01\x00\x05\x18\xd8N\x00\x00\x00\x00IEND\xaeB`\x82' > button_normal.png 2>/dev/null || true
fi

# ============================================================================
# STEP 5: Create client configuration
# ============================================================================
echo ""
echo "[6/7] Creating client configuration..."

# Create init.lua for client
cat > "$ROOT_DIR/client/init.lua" << 'EOF'
-- Shadow OT Client Initialization
-- This file is loaded first by the client

-- Protocol version
APP_VERSION = 1285
APP_NAME = "Shadow OT"

-- Default server
DEFAULT_HOST = "localhost"
DEFAULT_PORT = 7171

-- Asset paths
DATA_PATH = "data/"
SPRITES_PATH = DATA_PATH .. "things/"
UI_PATH = DATA_PATH .. "ui/"
FONTS_PATH = DATA_PATH .. "fonts/"

-- Client settings
VSYNC = true
ANTIALIAS = true
FULLSCREEN = false
WINDOW_WIDTH = 1024
WINDOW_HEIGHT = 768

-- Enable modules
ENABLE_BATTLE = true
ENABLE_CONSOLE = true
ENABLE_INVENTORY = true
ENABLE_MINIMAP = true
ENABLE_SKILLS = true

-- Shadow OT specific
ENABLE_BLOCKCHAIN = true
ENABLE_REALMS = true
ENABLE_MATCHMAKING = true
EOF

# Create modules initialization
cat > "$ROOT_DIR/client/modules/init.lua" << 'EOF'
-- Shadow OT Modules Initialization

-- Core game modules
dofile('game_battle/battle.lua')
dofile('game_console/console.lua')
dofile('game_inventory/inventory.lua')
dofile('game_minimap/minimap.lua')
dofile('game_skills/skills.lua')

-- Shadow OT specific modules
dofile('shadow_realms/realms.lua')
dofile('shadow_blockchain/blockchain.lua')
dofile('shadow_matchmaking/matchmaking.lua')

print("[Shadow OT] All modules loaded")
EOF

# ============================================================================
# STEP 6: Verify downloads and report
# ============================================================================
echo ""
echo "[7/7] Verifying assets..."

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                    ASSET DOWNLOAD REPORT                      ║"
echo "╠═══════════════════════════════════════════════════════════════╣"

# Check sprite files
if [ -f "$CLIENT_DATA/things/Tibia.dat" ]; then
    echo "║ ✅ Tibia.dat         - $(du -h "$CLIENT_DATA/things/Tibia.dat" | cut -f1) ║"
else
    echo "║ ❌ Tibia.dat         - MISSING                                ║"
fi

if [ -f "$CLIENT_DATA/things/Tibia.spr" ]; then
    echo "║ ✅ Tibia.spr         - $(du -h "$CLIENT_DATA/things/Tibia.spr" | cut -f1) ║"
else
    echo "║ ❌ Tibia.spr         - MISSING                                ║"
fi

# Check map files
if [ -f "$SERVER_DATA/maps/world.otbm" ]; then
    echo "║ ✅ world.otbm        - $(du -h "$SERVER_DATA/maps/world.otbm" | cut -f1) ║"
elif [ -f "$SERVER_DATA/maps/test.otbm" ]; then
    echo "║ ⚠️  test.otbm        - $(du -h "$SERVER_DATA/maps/test.otbm" | cut -f1) (fallback) ║"
else
    echo "║ ❌ Map files         - MISSING                                ║"
fi

# Check items
if [ -f "$SERVER_DATA/items/items.xml" ]; then
    echo "║ ✅ items.xml         - $(du -h "$SERVER_DATA/items/items.xml" | cut -f1) ║"
else
    echo "║ ❌ items.xml         - MISSING                                ║"
fi

# Count UI images
UI_COUNT=$(find "$CLIENT_DATA/ui/images" -name "*.png" 2>/dev/null | wc -l | tr -d ' ')
echo "║ 📦 UI Images         - $UI_COUNT files                              ║"

# Count fonts
FONT_COUNT=$(find "$CLIENT_DATA/fonts" -name "*.otfont" 2>/dev/null | wc -l | tr -d ' ')
echo "║ 📦 Fonts             - $FONT_COUNT files                              ║"

echo "╚═══════════════════════════════════════════════════════════════╝"

# Cleanup
rm -rf "$TEMP_DIR"

echo ""
echo "Asset download complete!"
echo ""
echo "Next steps:"
echo "  1. Rebuild client: cd client/build && cmake .. && make"
echo "  2. Run client: ./shadow-client"
echo "  3. Connect to server: localhost:7171"


