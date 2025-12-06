# Shadow OT

**The Ultimate Open OTServ Platform**

Shadow OT is a revolutionary, multi-realm Open OT server built with Rust for maximum performance and reliability. It features blockchain-native assets, cross-chain NFT support, and the most complete feature set of any OT server.

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
- **Rust Backend** - Ultra-fast, memory-safe game server
- **Low Latency** - Global edge servers for minimal ping
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

## Quick Start

### Prerequisites
- Rust 1.75+
- Docker & Docker Compose
- Node.js 20+
- PostgreSQL 16
- Redis 7

### Development Setup

```bash
# Clone the repository
git clone https://github.com/psavelis/shadow-ot.git
cd shadow-ot

# Start infrastructure
docker-compose -f docker/docker-compose.yml up -d postgres redis

# Run database migrations
cd crates/shadow-db
cargo sqlx migrate run

# Start the server
cargo run --release --bin shadow-server

# In another terminal, start the web frontend
cd web/landing
npm install
npm run dev
```

### Production Deployment

```bash
# Build Docker images
docker build -f docker/Dockerfile.server -t shadow-ot/server .
docker build -f web/landing/Dockerfile -t shadow-ot/web ./web/landing

# Deploy to Kubernetes
kubectl apply -k k8s/overlays/production
```

## Project Structure

```
shadow-ot/
├── crates/
│   ├── shadow-core/        # Core game engine
│   ├── shadow-protocol/    # OTServ Tib a protocol implementation
│   ├── shadow-db/          # Database layer
│   ├── shadow-api/         # REST/WebSocket API
│   ├── shadow-world/       # World/map management
│   ├── shadow-combat/      # Combat system
│   ├── shadow-realm/       # Realm configuration
│   ├── shadow-matchmaking/ # PvP matchmaking
│   ├── shadow-anticheat/   # Anti-cheat system
│   ├── shadow-scripting/   # Lua scripting
│   └── shadow-blockchain/  # Blockchain integration
├── web/
│   ├── landing/            # Main website (Next.js)
│   ├── dashboard/          # Player dashboard
│   ├── admin/              # Admin panel
│   ├── forum/              # Community forum
│   └── mapmaker/           # Map creation tool
├── k8s/                    # Kubernetes manifests
├── docker/                 # Docker configurations
├── data/                   # Game data files
├── realms/                 # Realm configurations
├── assets/                 # Sprites, maps, sounds (see assets/ASSETS.md)
└── scripts/                # Utility scripts
```

## Assets

Shadow OT uses open source assets from the OT community. Download assets:

```bash
./scripts/download-assets.sh
```

See [assets/ASSETS.md](assets/ASSETS.md) for full documentation on:
- Open source sprite repositories
- Map sources and editors
- Sound effect libraries
- How to create custom assets

## Configuration

### Server Configuration
```toml
# config/server.toml
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

API documentation is available at `/docs/api` when running the server, or visit [api.shadow-ot.com/docs](https://api.shadow-ot.com/docs).

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## Links

- **Website**: [shadow-ot.com](https://shadow-ot.com)
- **Discord**: [discord.gg/shadowot](https://discord.gg/shadowot)
- **Documentation**: [docs.shadow-ot.com](https://docs.shadow-ot.com)
- **API**: [api.shadow-ot.com](https://api.shadow-ot.com)

---

Built with ❤️ by the Shadow OT Team
