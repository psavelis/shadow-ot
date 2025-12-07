# Sprite Files for Shadow OT Client

## Required Files

The Shadow OT client requires sprite/appearance files to render the game world. You need **one** of the following setups:

### Option A: Traditional SPR/DAT Files (Recommended for OTClient)
```
Tibia.spr  - Sprite data file (compressed sprites)
Tibia.dat  - Thing type data file (item/creature properties)
```

### Option B: Appearances.dat (Protobuf Format)
```
appearances.dat  - Modern Tibia format (protobuf-encoded)
               - Already present in client/data/things/
```

## Where to Get Sprites

### Open Source Repositories

1. **OTClient Official Releases**
   - URL: https://github.com/edubart/otclient/releases
   - Contains: Tibia.spr, Tibia.dat for older versions
   
2. **OTClient Redemption**
   - URL: https://github.com/AoM-Tibia/OTClient-Redemption
   - Contains: Updated sprite files

3. **TFS (The Forgotten Server) Data**
   - URL: https://github.com/otland/forgottenserver
   - Contains: Server-side data, some clients included

4. **OpenTibiaBR Canary Assets**
   - URL: https://github.com/opentibiabr/canary
   - Contains: Full world maps and data files

### Protocol Versions

| Protocol | Files Needed | Notes |
|----------|--------------|-------|
| 8.60 | Tibia.spr, Tibia.dat | Classic OT protocol |
| 10.98 | Tibia.spr, Tibia.dat | Most common OT protocol |
| 12.x+ | appearances.dat | Modern protobuf format |

## Manual Download Instructions

### For SPR/DAT files:
```bash
cd /path/to/shadow-ot/client/data/sprites

# Try OTClient releases (check for latest URL)
# You may need to download manually from:
# https://github.com/edubart/otclient/releases

# Or extract from existing OTClient installation
```

### For Appearances.dat:
```bash
# Already available at: client/data/things/appearances.dat
# This file can be used if the client supports protobuf sprites
```

## Client Configuration

The client loads sprites from these paths (in order):
1. `client/data/sprites/Tibia.spr` + `Tibia.dat`
2. `client/data/things/appearances.dat`

Configure in `client/modules/client_main/client_main.lua`:
```lua
-- Set sprite format
SPRITE_FORMAT = "spr"  -- or "appearances"
```

## Troubleshooting

### "No sprites found" error
- Ensure either Tibia.spr OR appearances.dat is present
- Check file permissions (readable by client)
- Verify protocol version matches sprite version

### "Invalid sprite signature" error
- Sprite file doesn't match expected protocol version
- Download sprites for the correct Tibia version

### Black rectangles in game
- Sprites loading but wrong format/version
- Check client protocol settings

## Legal Note

Shadow OT uses open-source assets only. Do not use copyrighted CipSoft assets.
Recommended sources are community-created or explicitly open-source licensed.

---

*Last Updated: December 7, 2025*

