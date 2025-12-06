# Shadow OT Client

A modern, high-performance Open Tibia client built on OTClient Redemption (mehah/otclient) with additional enhancements for Shadow OT.

## Features

### Core Engine
- **Modern C++20** - Full C++20 standard support with latest compiler optimizations
- **WebAssembly Support** - Browser-based client via Emscripten compilation
- **Mobile First** - Native mobile support for Android and iOS
- **OpenGL ES 2.0** - Cross-platform rendering with fallback support

### Performance Enhancements
- **Texture Atlas** - 1.4x-3x FPS improvement in heavy rendering scenes
- **Async Sprite Loading** - Non-blocking asset loading with atomic operations
- **Unity Build** - Faster compilation with vcpkg Manifest Mode
- **Optimized Layouts** - Improved updateChildrenIndexStates & updateLayout algorithms

### Shadow OT Specific
- **Multi-Realm Support** - Seamless realm switching with themed UI
- **Blockchain Integration** - NFT wallet connection and asset display
- **Modern UI** - Dark theme with customizable skins
- **Enhanced Chat** - Rich text, emojis, and inline media
- **Matchmaking UI** - Integrated queue and ranking display

## Building

### Prerequisites
- CMake 3.20+
- Clang 14+ or GCC 12+ (MSVC 2022 on Windows)
- vcpkg package manager
- OpenGL ES 2.0+ compatible GPU

### Quick Build (Linux/macOS)
```bash
./build.sh
```

### Windows Build
```powershell
./build.ps1
```

### Web Build (Emscripten)
```bash
./build-web.sh
```

### Android Build
```bash
./build-android.sh
```

## Module System

Shadow OT Client uses a modular architecture based on Lua scripts:

```
modules/
├── client_main/          # Core client initialization
├── client_terminal/      # In-game Lua terminal
├── game_battle/          # Battle list and targeting
├── game_console/         # Chat and console
├── game_interface/       # Main game HUD
├── game_inventory/       # Inventory management
├── game_market/          # Marketplace UI
├── game_minimap/         # Minimap and navigation
├── game_skills/          # Skills window
├── shadow_blockchain/    # NFT/Wallet integration
├── shadow_matchmaking/   # PvP matchmaking UI
├── shadow_realms/        # Realm selection
└── shadow_themes/        # Theme customization
```

## Configuration

### Client Settings
Edit `config.lua` for client preferences:

```lua
-- Graphics
VSYNC = true
ANTIALIASING = true
RENDER_SCALE = 1.0
TEXTURE_QUALITY = 'high'

-- Audio
MUSIC_VOLUME = 50
SOUND_VOLUME = 70

-- Network
RECONNECT_AUTO = true
PING_DISPLAY = true

-- Shadow OT
DEFAULT_REALM = 'shadowveil'
BLOCKCHAIN_ENABLED = true
```

## Directory Structure

```
client/
├── src/                  # C++ source files
│   ├── client/          # Client-specific code
│   ├── framework/       # Core framework
│   └── shadow/          # Shadow OT extensions
├── modules/             # Lua modules
├── data/                # Assets and resources
│   ├── fonts/           # TrueType fonts
│   ├── images/          # UI images
│   ├── sounds/          # Audio files
│   └── styles/          # UI stylesheets
└── docs/                # Documentation
```

## Protocol Support

- **Tibia 12.x-13.x** - Full modern protocol support
- **Extended Protocol** - Shadow OT specific extensions
- **WebSocket** - Browser client connectivity
- **XTEA + RSA** - Standard Tibia encryption

## Contributing

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## License

MIT License - See LICENSE file for details.

## Credits

- [OTClient Redemption](https://github.com/mehah/otclient) - Base client
- [OTClient](https://github.com/edubart/otclient) - Original client
- Shadow OT Team - Extensions and improvements
