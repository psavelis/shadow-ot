# Shadow OT - AI Agent Coordination

## Project Status: Alpha 0.0.002

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                        AGENT TASK BOARD - Dec 2025                              │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Agent #1 (Web/TypeScript)     ████████████████████ 100% ✅ COMPLETE           │
│  Agent #2 (Rust/Server)        ████████████░░░░░░░░  60% 🟡 IN PROGRESS        │
│  Agent #3 (C++/Client)         ██████████░░░░░░░░░░  50% 🟡 IN PROGRESS        │
│  Agent #4 (Assets/Data)        ████░░░░░░░░░░░░░░░░  20% 🔴 BLOCKING           │
│                                                                                 │
│  Overall Launch Readiness:     ████████████░░░░░░░░  60%                       │
└────────────────────────────────────────────────────────────────────────────────┘
```

---

## Agent #1: Web Frontend (TypeScript/Next.js) ✅ COMPLETE

**Owner:** psavelis (AI Agent #1)
**Status:** 100% Complete
**Last Updated:** 2025-12-06

### Deliverables Complete
- [x] Landing website (`web/landing/`) - 30 TSX files
- [x] Player dashboard (`web/dashboard/`) - 22 TSX files
- [x] Admin panel (`web/admin/`) - 9 TSX files
- [x] Forum system (`web/forum/`) - 6 TSX files
- [x] Map maker (`web/mapmaker/`) - 3 TSX files
- [x] Shared library (`web/shared/`) - 26 TSX, 29 TS files
  - 161 React Query hooks
  - Full TypeScript types
  - API client with all endpoints
  - Real-time WebSocket hooks
- [x] CI/CD workflow (`.github/workflows/web-ci.yml`)
- [x] ESLint configuration
- [x] Download page integration with downloads service

### No Further Action Required
Agent #1 is available for code review and integration support.

---

## Agent #2: Game Server (Rust) 🟡 IN PROGRESS

**Owner:** AI Agent #2
**Status:** 60% Complete
**Priority:** HIGH

### Crates Status
| Crate | Lines | Status |
|-------|-------|--------|
| shadow-core | 8,652 | ✅ Code complete |
| shadow-world | 8,919 | ✅ Code complete |
| shadow-combat | 3,767 | ✅ Code complete |
| shadow-protocol | 2,628 | ✅ Code complete |
| shadow-scripting | 2,513 | ✅ Code complete |
| shadow-matchmaking | 2,028 | ✅ Code complete |
| shadow-api | 1,509 | ✅ Code complete |
| shadow-anticheat | 1,343 | ✅ Code complete |
| shadow-realm | 1,356 | ✅ Code complete |
| shadow-assets | 3,169 | ✅ Code complete |
| shadow-blockchain | 720 | ✅ Code complete |
| shadow-db | 424 | ✅ Code complete |

**Total: 37,028 lines of Rust code**

### 🔴 IMMEDIATE TASKS

```bash
# Task 2.1: Build the server binary
cd /Users/psavelis/sources/psavelis/shadow-ot
cargo build --release

# Task 2.2: Run the server
./target/release/shadow-server

# Task 2.3: Test API health
curl http://localhost:8080/health

# Task 2.4: Test login protocol on port 7171
# (requires client or protocol tester)
```

### Acceptance Criteria
- [ ] `cargo build --release` succeeds without errors
- [ ] Server binary runs and listens on ports 7171, 7172, 8080
- [ ] `/health` endpoint returns 200 OK
- [ ] Login server accepts RSA handshake
- [ ] Game server accepts XTEA encrypted packets

---

## Agent #3: Game Client (C++) 🟡 IN PROGRESS

**Owner:** AI Agent #3
**Status:** 50% Complete
**Priority:** HIGH

### Current State
- Binary exists: `client/build/shadow-client` (778KB)
- Source: OTClient-based with Shadow OT modules
- Modules: 15 Lua modules (battle, console, inventory, etc.)

### Recent Work (This Session)
```
client/src/framework/core/application.cpp  +4 lines
client/src/framework/core/application.h    +3 lines
client/src/framework/graphics/graphics.cpp +13 lines
client/src/framework/graphics/graphics.h   +3 lines
client/src/main.cpp                        +35 lines
```

### 🔴 IMMEDIATE TASKS

```bash
# Task 3.1: Rebuild client with latest changes
cd /Users/psavelis/sources/psavelis/shadow-ot/client
mkdir -p build && cd build
cmake ..
make -j$(nproc)

# Task 3.2: Run client
./shadow-client

# Task 3.3: Configure client to connect to server
# Edit data/config.lua or modules/client_main/client_main.lua
# Set: SERVER_HOST = "localhost"
#      SERVER_PORT = 7171
```

### Acceptance Criteria
- [ ] Client compiles without errors
- [ ] Client launches and shows login screen
- [ ] Client connects to server on localhost:7171
- [ ] RSA/XTEA handshake completes
- [ ] Character list displays after login

---

## Agent #4: Assets & Data 🟡 IN PROGRESS

**Owner:** AI Agent #4 (or manual)
**Status:** 60% Complete
**Priority:** HIGH

### What's Now Available

#### ✅ Map Files (OTBM) - COMPLETE
```
Location: data/maps/
Files:
  - canary.otbm (19.7MB) - Full Canary world map
  - forgotten.otbm (3.4MB) - TFS test map
```

#### ✅ Server Data - COMPLETE
```
Location: data/items/, client/data/things/
Files:
  - items.xml (3.5MB) - Item definitions
  - items.otb (2.3MB) - Item database
  - appearances.dat (4.5MB) - Item/creature appearances
```

#### ✅ Game Data JSON - COMPLETE
```
Location: data/
  - items/items.json - Items (759 lines)
  - monsters/monsters.json - Monsters (696 lines)
  - npcs/npcs.json - NPCs (453 lines)
  - spells/spells.json - Spells (567 lines)
  - quests/quests.json - Quests (367 lines)
  - vocations/vocations.json - Vocations (NEW)
  - achievements/achievements.json - Achievements (NEW)
```

### 🔴 Still Missing

#### 1. Sprite Files (SPR) - CRITICAL
```
Location: client/data/things/
Current: No Tibia.spr file
Required: Tibia.spr for sprite rendering
```

**Sources:**
- TibiaMaps: https://tibiamaps.github.io/tibia-map-data/
- Canary: https://github.com/opentibiabr/canary/tree/main/data/world
- Custom: Create with Remere's Map Editor

**Task 4.1:**
```bash
# Download from TibiaMaps
cd /Users/psavelis/sources/psavelis/shadow-ot/data/maps
curl -LO https://github.com/tibiamaps/tibia-map-data/raw/master/minimap-with-markers.zip
unzip minimap-with-markers.zip

# OR download from Canary
git clone --depth 1 https://github.com/opentibiabr/canary.git /tmp/canary
cp /tmp/canary/data/world/*.otbm ./
```

#### 2. Sprite Files (SPR/DAT) - CRITICAL
```
Location: client/data/sprites/ OR assets/sprites/
Current: EMPTY
Required: Tibia.spr, Tibia.dat, Tibia.pic
```

**Sources:**
- OTClient releases: https://github.com/mehah/otclient/releases
- Tibia installation (legal gray area)
- TibiaMaps: https://tibiamaps.github.io/tibia-map-data/mapper-sprites/

**Task 4.2:**
```bash
# Download from configured downloads service
cd /Users/psavelis/sources/psavelis/shadow-ot/client/data/sprites
curl -LO https://raw.githubusercontent.com/tibiamaps/tibia-map-data/master/mapper-sprites/Tibia.pic
curl -LO https://raw.githubusercontent.com/tibiamaps/tibia-map-data/master/mapper-sprites/Tibia.spr

# Note: Full DAT/SPR may need OTClient release
curl -LO https://github.com/mehah/otclient/releases/download/v1.0.1/otclient-windows-x64.zip
unzip otclient-windows-x64.zip -d /tmp/otclient
cp /tmp/otclient/data/*.spr /tmp/otclient/data/*.dat ./
```

#### 3. Additional Data Files (Lower Priority)
```
data/vocations/     - EMPTY (need vocations.json)
data/achievements/  - EMPTY (need achievements.json)
data/mounts/        - EMPTY (need mounts.json)
data/outfits/       - EMPTY (need outfits.json)
```

### Acceptance Criteria
- [ ] `data/maps/` contains at least 1 OTBM file
- [ ] `client/data/sprites/` contains Tibia.spr, Tibia.dat
- [ ] Client can load map data
- [ ] Client can render sprites

---

## E2E Integration Test 🔴 FINAL GATE

**Owner:** All Agents
**Status:** Not Started
**Priority:** CRITICAL (Launch Blocker)

### Full Flow Test

```bash
# 1. Start database (PostgreSQL + Redis)
docker-compose -f docker/docker-compose.yml up -d db redis

# 2. Run migrations
cd crates/shadow-db
sqlx migrate run

# 3. Start server
./target/release/shadow-server

# 4. Start client
./client/build/shadow-client

# 5. Test flow:
#    a. Create account via web (localhost:3000/register)
#    b. Launch client
#    c. Login with account
#    d. Create character
#    e. Enter game world
#    f. Move character
#    g. Verify server logs
```

### Acceptance Criteria
- [ ] Account creation works via web
- [ ] Client login succeeds
- [ ] Character list shows
- [ ] Character creation works
- [ ] World loads in client
- [ ] Character can move
- [ ] Server logs show player events

---

## Communication Protocol

### File-Based Coordination
- `AGENTS.md` - This file (task board)
- `PRD.md` - Product requirements and status
- Git commits - Use conventional commits with agent ID

### Commit Format
```
feat(scope): description

Agent: #N
Status: component status
```

### Branch Strategy
- `main` - Protected, requires PR
- `infra/*` - Infrastructure changes
- `feat/*` - Feature development
- `fix/*` - Bug fixes

---

## Next Actions Summary

| Priority | Agent | Task | Blocker? |
|----------|-------|------|----------|
| 🔴 P0 | #4 | Download OTBM map files | YES |
| 🔴 P0 | #4 | Download SPR/DAT sprites | YES |
| 🟡 P1 | #2 | `cargo build --release` | NO |
| 🟡 P1 | #2 | Start server, test ports | NO |
| 🟡 P1 | #3 | Rebuild client | NO |
| 🟡 P1 | #3 | Test client launch | NO |
| 🟢 P2 | ALL | E2E integration test | After P0/P1 |
| ✅ Done | #1 | Web frontend | NO |

---

*Last Updated: 2025-12-06*
*Tag: 0.0.0-alpha-0x0-a-0.0.002*

