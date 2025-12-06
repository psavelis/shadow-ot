#!/bin/bash
# Shadow OT Asset Downloader
# Downloads open source assets from community repositories

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR="$PROJECT_ROOT/assets"
DATA_DIR="$PROJECT_ROOT/data"
TMP_DIR="/tmp/shadow-ot-assets"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     Shadow OT Asset Downloader         ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo

# Create directories
mkdir -p "$ASSETS_DIR"/{sprites,maps,sounds,effects}
mkdir -p "$DATA_DIR"/{items,monsters,npcs,spells,quests}
mkdir -p "$TMP_DIR"

cleanup() {
    echo -e "\n${YELLOW}Cleaning up temporary files...${NC}"
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

# Function to download with progress
download_repo() {
    local name=$1
    local url=$2
    local dest=$3
    
    echo -e "${BLUE}Downloading $name...${NC}"
    if [ -d "$dest" ]; then
        echo -e "${YELLOW}  Already exists, pulling updates...${NC}"
        cd "$dest" && git pull --quiet
    else
        git clone --depth 1 --quiet "$url" "$dest"
    fi
    echo -e "${GREEN}  ✓ $name downloaded${NC}"
}

# Function to copy with structure
copy_assets() {
    local src=$1
    local dest=$2
    local pattern=$3
    
    if [ -d "$src" ]; then
        find "$src" -name "$pattern" -type f -exec cp {} "$dest" \; 2>/dev/null || true
    fi
}

echo -e "${YELLOW}Step 1/5: Downloading TFS Assets${NC}"
download_repo "TFS Assets" "https://github.com/otland/assets.git" "$TMP_DIR/tfs-assets"

if [ -d "$TMP_DIR/tfs-assets" ]; then
    echo "  Copying sprites..."
    copy_assets "$TMP_DIR/tfs-assets/sprites" "$ASSETS_DIR/sprites" "*.spr"
    copy_assets "$TMP_DIR/tfs-assets/sprites" "$ASSETS_DIR/sprites" "*.dat"
    copy_assets "$TMP_DIR/tfs-assets/sprites" "$ASSETS_DIR/sprites" "*.png"
    
    echo "  Copying maps..."
    copy_assets "$TMP_DIR/tfs-assets/maps" "$ASSETS_DIR/maps" "*.otbm"
    copy_assets "$TMP_DIR/tfs-assets/maps" "$ASSETS_DIR/maps" "*.otb"
fi

echo -e "\n${YELLOW}Step 2/5: Downloading OTClient Data${NC}"
download_repo "OTClient" "https://github.com/edubart/otclient.git" "$TMP_DIR/otclient"

if [ -d "$TMP_DIR/otclient/data" ]; then
    echo "  Copying client sprites..."
    copy_assets "$TMP_DIR/otclient/data/sprites" "$ASSETS_DIR/sprites" "*.png"
    
    echo "  Copying effects..."
    copy_assets "$TMP_DIR/otclient/data/effects" "$ASSETS_DIR/effects" "*"
    
    echo "  Copying sounds..."
    copy_assets "$TMP_DIR/otclient/data/sounds" "$ASSETS_DIR/sounds" "*.ogg"
    copy_assets "$TMP_DIR/otclient/data/sounds" "$ASSETS_DIR/sounds" "*.wav"
fi

echo -e "\n${YELLOW}Step 3/5: Downloading Canary Server Data${NC}"
download_repo "Canary" "https://github.com/opentibiabr/canary.git" "$TMP_DIR/canary"

if [ -d "$TMP_DIR/canary/data" ]; then
    echo "  Copying item definitions..."
    if [ -d "$TMP_DIR/canary/data/items" ]; then
        cp -r "$TMP_DIR/canary/data/items"/* "$DATA_DIR/items/" 2>/dev/null || true
    fi
    
    echo "  Copying monster data..."
    if [ -d "$TMP_DIR/canary/data/monster" ]; then
        cp -r "$TMP_DIR/canary/data/monster"/* "$DATA_DIR/monsters/" 2>/dev/null || true
    fi
    
    echo "  Copying NPC data..."
    if [ -d "$TMP_DIR/canary/data/npc" ]; then
        cp -r "$TMP_DIR/canary/data/npc"/* "$DATA_DIR/npcs/" 2>/dev/null || true
    fi
    
    echo "  Copying spell data..."
    if [ -d "$TMP_DIR/canary/data/spells" ]; then
        cp -r "$TMP_DIR/canary/data/spells"/* "$DATA_DIR/spells/" 2>/dev/null || true
    fi
fi

echo -e "\n${YELLOW}Step 4/5: Downloading RME (Map Editor)${NC}"
download_repo "RME" "https://github.com/hjnilsson/rme.git" "$TMP_DIR/rme"

if [ -d "$TMP_DIR/rme/data" ]; then
    echo "  Copying editor data..."
    copy_assets "$TMP_DIR/rme/data" "$ASSETS_DIR/maps" "*.otb"
fi

echo -e "\n${YELLOW}Step 5/5: Downloading Kenney Assets${NC}"
echo "  Note: Kenney assets require manual download from https://kenney.nl/assets"
echo "  Recommended packs:"
echo "    - RPG Urban Pack"
echo "    - Game Icons"
echo "    - UI Pack"

# Create placeholder directories for manual assets
mkdir -p "$ASSETS_DIR/sprites/kenney"
mkdir -p "$ASSETS_DIR/sounds/kenney"

cat > "$ASSETS_DIR/sprites/kenney/README.md" << 'EOF'
# Kenney Assets

Download assets from https://kenney.nl/assets

Recommended:
- RPG Urban Pack: https://kenney.nl/assets/rpg-urban-pack
- Game Icons: https://kenney.nl/assets/game-icons
- UI Pack: https://kenney.nl/assets/ui-pack

All Kenney assets are CC0 (Public Domain).
EOF

# Summary
echo
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║         Download Complete!             ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo
echo "Asset locations:"
echo "  Sprites: $ASSETS_DIR/sprites/"
echo "  Maps:    $ASSETS_DIR/maps/"
echo "  Sounds:  $ASSETS_DIR/sounds/"
echo "  Effects: $ASSETS_DIR/effects/"
echo "  Data:    $DATA_DIR/"
echo
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Run sprite extraction: cargo run --bin sprite-extractor"
echo "  2. Convert OTB files: cargo run --bin otb-converter"
echo "  3. Download Kenney assets manually (see assets/sprites/kenney/README.md)"
echo
echo -e "${BLUE}See assets/ASSETS.md for full documentation.${NC}"
