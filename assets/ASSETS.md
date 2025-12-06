# Shadow OT Assets

This document lists all asset sources and how to obtain/generate them for Shadow OT.

## Open Source Asset Sources

### Sprites & Graphics

#### TFS (The Forgotten Server) Assets
- **Repository**: https://github.com/otland/assets
- **License**: Custom (OT community)
- **Contents**: Full sprite sheets (.spr), item definitions (.dat), maps (.otbm)
- **Usage**: Base sprites for items, creatures, outfits

#### OpenTibia Sprites
- **Repository**: https://github.com/opentibia/opentibia
- **Contains**: Classic OT sprite collection
- **Format**: .spr, .dat files

#### OTClient Sprites
- **Repository**: https://github.com/edubart/otclient
- **Contents**: Client-compatible sprites and graphics
- **Path**: `data/sprites/`

#### Canary Server Assets
- **Repository**: https://github.com/opentibiabr/canary
- **Contents**: Modern sprites, items, creatures
- **Path**: `data/` directory

### Maps

#### Global Tibia Map
- **Repository**: https://github.com/nicklauslittle/tibia-map-data
- **Contents**: Comprehensive map data with spawn information
- **Format**: OTBM compatible

#### TFS Maps Collection
- **Repository**: https://github.com/otland/mapeditor
- **Contents**: Sample maps and map editor
- **Format**: .otbm files

#### Custom Map Resources
- **RME (Remere's Map Editor)**: https://github.com/hjnilsson/rme
- **Usage**: Create and edit .otbm maps

### Sounds

#### OpenGameArt
- **Website**: https://opengameart.org
- **License**: Various (CC0, CC-BY, etc.)
- **Search**: "rpg sounds", "fantasy sfx"

#### Freesound
- **Website**: https://freesound.org
- **License**: Various Creative Commons
- **Tags**: battle, magic, footsteps, ambient

#### Kenney Assets
- **Website**: https://kenney.nl/assets
- **License**: CC0 (Public Domain)
- **Contents**: UI sounds, RPG effects

### Effects & Particles

#### OTClient Effects
- **Repository**: https://github.com/edubart/otclient
- **Path**: `data/effects/`
- **Format**: Effect definitions and sprites

#### OpenGameArt Effects
- **Website**: https://opengameart.org/art-search-advanced?keys=particle
- **Contents**: Particle effects, magic spells

### Artwork & UI

#### OpenGameArt UI
- **Website**: https://opengameart.org/art-search-advanced?keys=ui+fantasy
- **Contents**: UI elements, buttons, frames

#### Game-Icons.net
- **Website**: https://game-icons.net
- **License**: CC-BY 3.0
- **Contents**: 4000+ game icons for skills, items, etc.

## Asset Structure

```
assets/
├── sprites/
│   ├── items/           # Item sprites (.png)
│   ├── creatures/       # Monster/NPC sprites
│   ├── outfits/         # Character outfit sprites
│   ├── effects/         # Spell/effect animations
│   └── ui/              # UI elements
├── maps/
│   ├── *.otbm           # Map files
│   └── *.otb            # Item definitions
├── sounds/
│   ├── ambient/         # Background sounds
│   ├── effects/         # SFX
│   └── music/           # BGM
└── effects/
    ├── particles/       # Particle definitions
    └── shaders/         # Visual shaders
```

## Downloading Assets

### Automated Download

Run the asset download script:

```bash
./scripts/download-assets.sh
```

### Manual Download

1. **TFS Assets**:
   ```bash
   git clone https://github.com/otland/assets.git /tmp/tfs-assets
   cp -r /tmp/tfs-assets/sprites/* assets/sprites/
   cp -r /tmp/tfs-assets/maps/* assets/maps/
   ```

2. **OTClient Sprites**:
   ```bash
   git clone https://github.com/edubart/otclient.git /tmp/otclient
   cp -r /tmp/otclient/data/sprites/* assets/sprites/
   cp -r /tmp/otclient/data/effects/* assets/effects/
   ```

3. **Canary Data**:
   ```bash
   git clone https://github.com/opentibiabr/canary.git /tmp/canary
   cp -r /tmp/canary/data/items/* data/items/
   cp -r /tmp/canary/data/monster/* data/monsters/
   cp -r /tmp/canary/data/npc/* data/npcs/
   ```

## Asset Conversion

### SPR/DAT to PNG

Use the sprite extractor tool:

```bash
cargo run --bin sprite-extractor -- \
    --spr assets/sprites/Tibia.spr \
    --dat assets/sprites/Tibia.dat \
    --output assets/sprites/extracted/
```

### OTB to JSON

Convert OTB item definitions:

```bash
cargo run --bin otb-converter -- \
    --input assets/maps/items.otb \
    --output data/items/items.json
```

## Creating Custom Assets

### Sprite Requirements

- **Format**: PNG with transparency
- **Size**: 32x32 pixels (standard), 64x64 (large creatures)
- **Naming**: `<type>_<id>.png` (e.g., `item_2160.png`)

### Map Requirements

- **Format**: OTBM (Open Tibia Binary Map)
- **Editor**: RME (Remere's Map Editor)
- **Versions**: Support OTBM version 1-3

### Sound Requirements

- **Format**: OGG Vorbis (recommended), WAV, MP3
- **Sample Rate**: 44.1kHz
- **Channels**: Stereo for music, Mono for SFX

## Legal Notice

Shadow OT does not include copyrighted assets. Users must:

1. Use open source assets from the listed repositories
2. Create their own original assets
3. Obtain proper licenses for any third-party assets

All asset repositories listed here are open source community projects.
The Shadow OT project respects intellectual property rights.

## Contributing Assets

To contribute assets:

1. Ensure you have rights to the assets
2. Use open formats (PNG, OGG, JSON)
3. Follow naming conventions
4. Include license information
5. Submit PR to the repository

## Reference Repositories

| Repository | Description | License |
|------------|-------------|---------|
| [otland/assets](https://github.com/otland/assets) | TFS assets | Community |
| [edubart/otclient](https://github.com/edubart/otclient) | OTClient data | MIT |
| [opentibiabr/canary](https://github.com/opentibiabr/canary) | Canary server | GPL-3.0 |
| [hjnilsson/rme](https://github.com/hjnilsson/rme) | Map editor | GPL-2.0 |
| [kenney.nl](https://kenney.nl/assets) | Game assets | CC0 |
| [opengameart.org](https://opengameart.org) | Various | Various CC |
| [game-icons.net](https://game-icons.net) | Icons | CC-BY 3.0 |
