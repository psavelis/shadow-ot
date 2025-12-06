#!/bin/bash
# Shadow OT Asset Downloader
# Downloads real Tibia assets from open source OTClient repositories

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIENT_DIR="$(dirname "$SCRIPT_DIR")"
DATA_DIR="$CLIENT_DIR/data"
TEMP_DIR="$CLIENT_DIR/.asset_temp"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Shadow OT Asset Downloader${NC}"
echo "=================================="

# Create directories
mkdir -p "$DATA_DIR/sprites"
mkdir -p "$DATA_DIR/fonts"
mkdir -p "$DATA_DIR/ui"
mkdir -p "$DATA_DIR/sounds"
mkdir -p "$DATA_DIR/music"
mkdir -p "$TEMP_DIR"

# Function to download from GitHub
download_github_file() {
    local repo=$1
    local path=$2
    local dest=$3
    local branch=${4:-main}

    local url="https://raw.githubusercontent.com/$repo/$branch/$path"
    echo -e "${YELLOW}Downloading:${NC} $path"

    if curl -sL -o "$dest" "$url"; then
        echo -e "${GREEN}  ✓${NC} Downloaded successfully"
        return 0
    else
        echo -e "${RED}  ✗${NC} Failed to download"
        return 1
    fi
}

# Download OTClient Redemption UI assets
echo -e "\n${GREEN}Downloading OTClient UI assets...${NC}"
OTCLIENT_REPO="mehah/otclient"

# Core UI images
download_github_file "$OTCLIENT_REPO" "data/images/background.png" "$DATA_DIR/ui/background.png" || true
download_github_file "$OTCLIENT_REPO" "data/images/clienticon.png" "$DATA_DIR/ui/clienticon.png" || true

# Download fonts
echo -e "\n${GREEN}Downloading fonts...${NC}"
download_github_file "$OTCLIENT_REPO" "data/fonts/verdana-11px-rounded.otfont" "$DATA_DIR/fonts/verdana-11px-rounded.otfont" || true
download_github_file "$OTCLIENT_REPO" "data/fonts/verdana-11px-antialised.otfont" "$DATA_DIR/fonts/verdana-11px-antialised.otfont" || true

# Download styles
echo -e "\n${GREEN}Downloading UI styles...${NC}"
download_github_file "$OTCLIENT_REPO" "data/styles/10-scrollbar.otui" "$DATA_DIR/ui/scrollbar.otui" || true
download_github_file "$OTCLIENT_REPO" "data/styles/20-buttons.otui" "$DATA_DIR/ui/buttons.otui" || true

# Download from edubart's OTClient (original)
echo -e "\n${GREEN}Downloading from edubart/otclient...${NC}"
EDUBART_REPO="edubart/otclient"
download_github_file "$EDUBART_REPO" "data/images/background.png" "$DATA_DIR/ui/background_original.png" "master" || true

# Notify about DAT/SPR files
echo -e "\n${YELLOW}=================================${NC}"
echo -e "${YELLOW}IMPORTANT: DAT/SPR Files${NC}"
echo -e "${YELLOW}=================================${NC}"
echo ""
echo "DAT and SPR files contain Tibia's proprietary graphics."
echo "You need to obtain these from a legal Tibia installation:"
echo ""
echo "1. Download Tibia client from tibia.com"
echo "2. Copy Tibia.dat and Tibia.spr to: $DATA_DIR/"
echo ""
echo "Or use extracted open-source assets compatible with protocol 10.98+"
echo ""

# Create placeholder files for testing
echo -e "\n${GREEN}Creating placeholder asset info...${NC}"
cat > "$DATA_DIR/ASSETS_README.md" << 'EOF'
# Shadow OT Client Assets

## Required Files

### Tibia Data Files
- `Tibia.dat` - Item/creature/effect metadata (from Tibia client)
- `Tibia.spr` - Sprite graphics data (from Tibia client)

### Server Data Files
- `items.otb` - Item database (from server data)
- `items.xml` - Item definitions

## Asset Sources

### Open Source Clients
- [OTClient Redemption](https://github.com/mehah/otclient) - Modern client
- [OTClient Original](https://github.com/edubart/otclient) - Original OTClient

### Server Data
- [OpenTibiaBR Canary](https://github.com/opentibiabr/canary) - Full server
- [TFS](https://github.com/otland/forgottenserver) - The Forgotten Server

## Protocol Versions

| Version | DAT Signature | SPR Signature | Description |
|---------|---------------|---------------|-------------|
| 10.98   | 0x57D6B7C3   | 0x57D6B7C3   | Most OT servers |
| 12.00   | 0x5C5A1B8A   | 0x5C5A1B8A   | Modern features |
| 12.85   | 0x60A29E59   | 0x60A29E59   | Bosstiary update |

## Shadow OT Extensions

Shadow OT adds custom sprites for:
- Blockchain wallet UI
- Realm selection interface
- NFT equipment display
- Shadow-exclusive items
EOF

echo -e "\n${GREEN}Asset download complete!${NC}"
echo "Check $DATA_DIR/ASSETS_README.md for more information."

# Cleanup
rm -rf "$TEMP_DIR"
