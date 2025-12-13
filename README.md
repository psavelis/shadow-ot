# Shadow OT

**The Ultimate Open Tibia Server Platform**

Shadow OT is a modern, multi-realm Open Tibia server built with Rust for maximum performance and reliability. It features blockchain-native assets, cross-chain NFT support, and the most complete feature set of any OT server.

## Quick Start

```bash
# 1. Download sprite assets (required for game client)
make sprites

# 2. Start all services
make up

# 3. Check status
make status
```

That's it! All services will be available at:

| Service | URL |
|---------|-----|
| Web Frontend | http://localhost:3000 |
| Admin Panel | http://localhost:3001 |
| REST API | http://localhost:8080 |
| API Health | http://localhost:8080/health |
| WebSocket | ws://localhost:8081 |
| Login Server | localhost:7171 |
| Game Server | localhost:7172 |
| Grafana | http://localhost:3002 |
| Prometheus | http://localhost:9091 |

## Prerequisites

- **Docker** and **Docker Compose** (required)
- **Make** (for convenience commands)
- ~500MB disk space for sprite assets
- ~2GB disk space for Docker images

## Available Commands

```bash
# === Quick Start (Docker Compose) ===
make up          # Start all services
make down        # Stop all services
make status      # Show service status and URLs
make logs        # Follow all service logs
make health      # Check service health

# === Database ===
make db-migrate  # Apply database migrations
make db-reset    # Reset database (WARNING: destroys data)

# === Assets ===
make sprites     # Download sprite files for game client

# === Kubernetes (Advanced) ===
make k8s-up      # Start with Kind + MetalLB
make k8s-down    # Stop Kubernetes cluster

# === Help ===
make help        # Show all available commands
```

## Features

### Core Game Features
- **Multi-Realm Architecture** - One account, multiple themed game worlds
- **Complete OTServ Features** - All features from real OTServ and more
- **Houses & Guilds** - Full housing system with auctions and guild halls
- **Market System** - In-game market with cross-realm trading
- **Bestiary & Prey** - Complete creature tracking and hunting systems
- **Achievements** - Hundreds of achievements with exclusive rewards
- **Quests** - Extensive quest system with custom content support

### Technical Excellence
- **Rust Backend** - Ultra-fast, memory-safe game server (54K+ lines)
- **Low Latency** - Optimized for minimal ping
- **Multi-Protocol** - Support for clients 8.6 to 12.x+
- **Hot Reload** - Update realm configs without downtime
- **Kubernetes Native** - Scalable, resilient infrastructure

### Blockchain Integration
- **NFT Assets** - Mint items, houses, achievements as NFTs
- **Multi-Chain** - Ethereum, Polygon, Starknet, Bitcoin, Spark
- **Cross-Chain Bridge** - Transfer assets between chains
- **Wallet Login** - Secure Web3 authentication

### Community Features
- **Forums** - Integrated discussion boards
- **Matchmaking** - Ranked PvP with ELO ratings
- **Custom Content** - User-submitted maps and monsters
- **Anti-Cheat** - AI-powered detection system

## Realms

| Realm | Theme | PvP Type | Rates |
|-------|-------|----------|-------|
| Shadowveil | Dark & Mysterious | Open PvP | 5x |
| Aetheria | Mythic & Epic | Optional PvP | 3x |
| Warbound | PvP Focused | Hardcore PvP | 10x |
| Mythara | Classic Experience | Retro Open | 1x |
| Voidborne | Seasonal | Mixed | 7x |

## Project Structure

```
shadow-ot/
├── crates/                    # Rust backend (12 crates, 54K+ lines)
│   ├── shadow-core/           # Core game engine
│   ├── shadow-protocol/       # OTServ protocol implementation
│   ├── shadow-db/             # Database layer + migrations
│   ├── shadow-api/            # REST/WebSocket API (80+ endpoints)
│   ├── shadow-world/          # World/map management
│   ├── shadow-combat/         # Combat system
│   ├── shadow-realm/          # Realm configuration
│   ├── shadow-matchmaking/    # PvP matchmaking
│   ├── shadow-anticheat/      # Anti-cheat system
│   ├── shadow-scripting/      # Lua scripting
│   ├── shadow-assets/         # Asset pipeline
│   └── shadow-blockchain/     # Blockchain integration
├── web/                       # Frontend applications
│   ├── landing/               # Main website (Next.js)
│   ├── dashboard/             # Player dashboard
│   ├── admin/                 # Admin panel
│   ├── forum/                 # Community forum
│   ├── mapmaker/              # Map creation tool
│   └── shared/                # Shared components & hooks
├── client/                    # Game client (C++)
│   ├── src/                   # Source code
│   ├── data/                  # Client assets
│   │   ├── sprites/           # Tibia.spr, Tibia.dat
│   │   └── things/            # appearances.dat, items.otb
│   └── build/                 # Compiled binary
├── k8s/                       # Kubernetes manifests
│   ├── base/                  # Base configuration
│   └── overlays/              # Environment overlays
├── docker/                    # Docker configurations
│   ├── docker-compose.yml     # Local development stack
│   └── Dockerfile.server      # Server image
├── data/                      # Game data files (JSON)
│   ├── items/                 # Item definitions
│   ├── monsters/              # Monster definitions
│   ├── npcs/                  # NPC dialogues
│   ├── spells/                # Spell definitions
│   └── quests/                # Quest definitions
├── realms/                    # Realm configurations (TOML)
├── Makefile                   # Build & run commands
├── PRD.md                     # Product requirements
├── AGENTS.md                  # Project status
└── README.md                  # This file
```

## Development

### Running Individual Services

```bash
# Start only database services
docker compose -f docker/docker-compose.yml up -d postgres redis

# View server logs
make logs-server

# View web logs
make logs-web
```

### Building from Source

```bash
# Build server (requires Rust 1.75+)
cargo build --release --bin shadow-server

# Build client (requires CMake, C++20)
cd client/build && cmake .. && make

# Build web (requires Node.js 20+)
cd web/landing && npm install && npm run build
```

### Database

Migrations are in `crates/shadow-db/migrations/`. Apply them with:

```bash
make db-migrate
```

The database schema includes 83 tables covering accounts, characters, guilds, houses, market, achievements, and more.

## Configuration

### Server Configuration
```toml
# config/server.toml (or docker/config/server.toml)
[server]
name = "Shadow OT"
max_players_global = 10000

[network]
login_port = 7171
game_port_start = 7172

[features]
multi_realm = true
blockchain = true
```

### Realm Configuration
```toml
# realms/shadowveil/config.toml
[realm]
name = "Shadowveil"
theme = "dark"
max_players = 500

[rates]
experience = 5.0
skill = 3.0
loot = 2.0

[pvp]
type = "open"
protection_level = 50
```

## API Documentation

When the server is running, API documentation is available at:
- Swagger UI: http://localhost:8080/docs
- OpenAPI JSON: http://localhost:8080/openapi.json

## Troubleshooting

### Services won't start
```bash
# Check Docker is running
docker info

# Check for port conflicts
lsof -i :3000 -i :8080 -i :7171

# View logs
make logs
```

### Database connection issues
```bash
# Check PostgreSQL is healthy
docker exec shadow-postgres pg_isready -U shadow

# Reset database if needed
make db-reset
```

### Missing sprites
```bash
# Download sprites
make sprites

# Verify files exist
ls -la client/data/sprites/
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## Links

- **Documentation**: [PRD.md](PRD.md) - Full product requirements
- **Status**: [AGENTS.md](AGENTS.md) - Current project status
- **Discord**: [discord.gg/shadowot](https://discord.gg/shadowot)

---

Built with Rust, TypeScript, and C++ by the Shadow OT Team
