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
