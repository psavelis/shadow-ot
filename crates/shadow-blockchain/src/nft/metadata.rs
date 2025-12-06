//! NFT Metadata Generation
//!
//! Build and generate NFT metadata according to standards.

use serde::{Deserialize, Serialize};
use crate::{AssetType, Chain, NftAttribute, NftMetadata, NftProperties, Rarity, Result};

/// Builder for NFT metadata
pub struct MetadataBuilder {
    name: String,
    description: String,
    image: String,
    external_url: Option<String>,
    animation_url: Option<String>,
    attributes: Vec<NftAttribute>,
    game_id: String,
    realm_id: Option<uuid::Uuid>,
    asset_type: String,
    original_chain: Chain,
}

impl MetadataBuilder {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            image: String::new(),
            external_url: None,
            animation_url: None,
            attributes: vec![],
            game_id: "shadow-ot".to_string(),
            realm_id: None,
            asset_type: "unknown".to_string(),
            original_chain: Chain::Polygon,
        }
    }

    pub fn image(mut self, url: &str) -> Self {
        self.image = url.to_string();
        self
    }

    pub fn external_url(mut self, url: &str) -> Self {
        self.external_url = Some(url.to_string());
        self
    }

    pub fn animation_url(mut self, url: &str) -> Self {
        self.animation_url = Some(url.to_string());
        self
    }

    pub fn attribute(mut self, trait_type: &str, value: impl Into<serde_json::Value>) -> Self {
        self.attributes.push(NftAttribute {
            trait_type: trait_type.to_string(),
            value: value.into(),
            display_type: None,
        });
        self
    }

    pub fn attribute_with_display(
        mut self,
        trait_type: &str,
        value: impl Into<serde_json::Value>,
        display_type: &str,
    ) -> Self {
        self.attributes.push(NftAttribute {
            trait_type: trait_type.to_string(),
            value: value.into(),
            display_type: Some(display_type.to_string()),
        });
        self
    }

    pub fn realm(mut self, realm_id: uuid::Uuid) -> Self {
        self.realm_id = Some(realm_id);
        self
    }

    pub fn asset_type(mut self, asset_type: &str) -> Self {
        self.asset_type = asset_type.to_string();
        self
    }

    pub fn chain(mut self, chain: Chain) -> Self {
        self.original_chain = chain;
        self
    }

    pub fn build(self) -> NftMetadata {
        NftMetadata {
            name: self.name,
            description: self.description,
            image: self.image,
            external_url: self.external_url,
            animation_url: self.animation_url,
            attributes: self.attributes,
            properties: NftProperties {
                game_id: self.game_id,
                realm_id: self.realm_id,
                asset_type: self.asset_type,
                original_chain: self.original_chain,
                bridged_chains: vec![],
                created_at: chrono::Utc::now(),
                shadow_ot_version: env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }
}

/// Generate metadata from game assets
pub struct MetadataGenerator {
    base_image_url: String,
    external_base_url: String,
}

impl MetadataGenerator {
    pub fn new(base_image_url: &str, external_base_url: &str) -> Self {
        Self {
            base_image_url: base_image_url.to_string(),
            external_base_url: external_base_url.to_string(),
        }
    }

    /// Generate metadata for an in-game asset
    pub fn generate(&self, asset: &AssetType, chain: Chain) -> Result<NftMetadata> {
        match asset {
            AssetType::Item { item_id, name, rarity, attributes } => {
                self.generate_item_metadata(*item_id, name, *rarity, attributes, chain)
            }
            AssetType::Outfit { outfit_id, name, addons } => {
                self.generate_outfit_metadata(*outfit_id, name, *addons, chain)
            }
            AssetType::Mount { mount_id, name } => {
                self.generate_mount_metadata(*mount_id, name, chain)
            }
            AssetType::House { house_id, name, realm_id, size, location } => {
                self.generate_house_metadata(*house_id, name, *realm_id, *size, location, chain)
            }
            AssetType::Achievement { achievement_id, name, points } => {
                self.generate_achievement_metadata(*achievement_id, name, *points, chain)
            }
            AssetType::GuildAsset { guild_id, asset_type } => {
                self.generate_guild_metadata(*guild_id, *asset_type, chain)
            }
            AssetType::EventItem { event_id, item_id, name, event_name } => {
                self.generate_event_item_metadata(*event_id, *item_id, name, event_name, chain)
            }
            AssetType::Territory { realm_id, coordinates, name } => {
                self.generate_territory_metadata(*realm_id, *coordinates, name, chain)
            }
        }
    }

    fn generate_item_metadata(
        &self,
        item_id: u32,
        name: &str,
        rarity: Rarity,
        attributes: &[crate::ItemAttribute],
        chain: Chain,
    ) -> Result<NftMetadata> {
        let mut builder = MetadataBuilder::new(
            name,
            &format!("A {} item from Shadow OT", rarity_name(rarity)),
        )
        .image(&format!("{}/items/{}.png", self.base_image_url, item_id))
        .external_url(&format!("{}/items/{}", self.external_base_url, item_id))
        .asset_type("item")
        .chain(chain)
        .attribute("Item ID", serde_json::Value::Number(item_id.into()))
        .attribute("Rarity", serde_json::Value::String(rarity_name(rarity).to_string()));

        // Add item-specific attributes
        for attr in attributes {
            let value = match &attr.value {
                crate::AttributeValue::Number(n) => serde_json::Value::Number((*n).into()),
                crate::AttributeValue::String(s) => serde_json::Value::String(s.clone()),
                crate::AttributeValue::Boolean(b) => serde_json::Value::Bool(*b),
                crate::AttributeValue::Range { current, .. } => {
                    serde_json::Value::Number((*current).into())
                }
            };
            builder = builder.attribute(&attr.name, value);
        }

        Ok(builder.build())
    }

    fn generate_outfit_metadata(
        &self,
        outfit_id: u32,
        name: &str,
        addons: u8,
        chain: Chain,
    ) -> Result<NftMetadata> {
        let builder = MetadataBuilder::new(
            name,
            &format!("{} outfit with {} addon(s)", name, addons.count_ones()),
        )
        .image(&format!("{}/outfits/{}.png", self.base_image_url, outfit_id))
        .external_url(&format!("{}/outfits/{}", self.external_base_url, outfit_id))
        .asset_type("outfit")
        .chain(chain)
        .attribute("Outfit ID", serde_json::Value::Number(outfit_id.into()))
        .attribute("Addons", serde_json::Value::Number(addons.into()))
        .attribute("Has First Addon", serde_json::Value::Bool(addons & 1 != 0))
        .attribute("Has Second Addon", serde_json::Value::Bool(addons & 2 != 0));

        Ok(builder.build())
    }

    fn generate_mount_metadata(
        &self,
        mount_id: u32,
        name: &str,
        chain: Chain,
    ) -> Result<NftMetadata> {
        let builder = MetadataBuilder::new(name, &format!("{} mount from Shadow OT", name))
            .image(&format!("{}/mounts/{}.png", self.base_image_url, mount_id))
            .external_url(&format!("{}/mounts/{}", self.external_base_url, mount_id))
            .asset_type("mount")
            .chain(chain)
            .attribute("Mount ID", serde_json::Value::Number(mount_id.into()));

        Ok(builder.build())
    }

    fn generate_house_metadata(
        &self,
        house_id: u32,
        name: &str,
        realm_id: uuid::Uuid,
        size: u32,
        location: &str,
        chain: Chain,
    ) -> Result<NftMetadata> {
        let builder = MetadataBuilder::new(
            name,
            &format!("A {} sqm house in {} - {}", size, location, name),
        )
        .image(&format!("{}/houses/{}.png", self.base_image_url, house_id))
        .external_url(&format!("{}/houses/{}", self.external_base_url, house_id))
        .asset_type("house")
        .chain(chain)
        .realm(realm_id)
        .attribute("House ID", serde_json::Value::Number(house_id.into()))
        .attribute_with_display("Size", serde_json::Value::Number(size.into()), "number")
        .attribute("Location", serde_json::Value::String(location.to_string()));

        Ok(builder.build())
    }

    fn generate_achievement_metadata(
        &self,
        achievement_id: u32,
        name: &str,
        points: u32,
        chain: Chain,
    ) -> Result<NftMetadata> {
        let builder = MetadataBuilder::new(
            &format!("Achievement: {}", name),
            &format!("Earned by completing: {}", name),
        )
        .image(&format!(
            "{}/achievements/{}.png",
            self.base_image_url, achievement_id
        ))
        .external_url(&format!(
            "{}/achievements/{}",
            self.external_base_url, achievement_id
        ))
        .asset_type("achievement")
        .chain(chain)
        .attribute(
            "Achievement ID",
            serde_json::Value::Number(achievement_id.into()),
        )
        .attribute_with_display("Points", serde_json::Value::Number(points.into()), "number");

        Ok(builder.build())
    }

    fn generate_guild_metadata(
        &self,
        guild_id: uuid::Uuid,
        asset_type: crate::GuildAssetType,
        chain: Chain,
    ) -> Result<NftMetadata> {
        let type_name = match asset_type {
            crate::GuildAssetType::Emblem => "Emblem",
            crate::GuildAssetType::Banner => "Banner",
            crate::GuildAssetType::Territory => "Territory",
            crate::GuildAssetType::Treasury => "Treasury",
        };

        let builder = MetadataBuilder::new(
            &format!("Guild {}", type_name),
            &format!("Guild {} NFT", type_name),
        )
        .image(&format!(
            "{}/guilds/{}/{}.png",
            self.base_image_url, guild_id, type_name.to_lowercase()
        ))
        .asset_type("guild")
        .chain(chain)
        .attribute("Guild ID", serde_json::Value::String(guild_id.to_string()))
        .attribute("Asset Type", serde_json::Value::String(type_name.to_string()));

        Ok(builder.build())
    }

    fn generate_event_item_metadata(
        &self,
        event_id: uuid::Uuid,
        item_id: u32,
        name: &str,
        event_name: &str,
        chain: Chain,
    ) -> Result<NftMetadata> {
        let builder = MetadataBuilder::new(
            name,
            &format!("Special item from the {} event", event_name),
        )
        .image(&format!(
            "{}/events/{}/{}.png",
            self.base_image_url, event_id, item_id
        ))
        .asset_type("event_item")
        .chain(chain)
        .attribute("Event ID", serde_json::Value::String(event_id.to_string()))
        .attribute("Event Name", serde_json::Value::String(event_name.to_string()))
        .attribute("Item ID", serde_json::Value::Number(item_id.into()));

        Ok(builder.build())
    }

    fn generate_territory_metadata(
        &self,
        realm_id: uuid::Uuid,
        coordinates: (u16, u16, u16, u16),
        name: &str,
        chain: Chain,
    ) -> Result<NftMetadata> {
        let (x1, y1, x2, y2) = coordinates;
        let area = ((x2 - x1) as u32) * ((y2 - y1) as u32);

        let builder = MetadataBuilder::new(
            name,
            &format!("A {} sqm territory in Shadow OT", area),
        )
        .image(&format!(
            "{}/territories/{}.png",
            self.base_image_url, realm_id
        ))
        .asset_type("territory")
        .chain(chain)
        .realm(realm_id)
        .attribute(
            "Coordinates",
            serde_json::Value::String(format!("({}, {}) to ({}, {})", x1, y1, x2, y2)),
        )
        .attribute_with_display("Area", serde_json::Value::Number(area.into()), "number");

        Ok(builder.build())
    }
}

fn rarity_name(rarity: Rarity) -> &'static str {
    match rarity {
        Rarity::Common => "Common",
        Rarity::Uncommon => "Uncommon",
        Rarity::Rare => "Rare",
        Rarity::Epic => "Epic",
        Rarity::Legendary => "Legendary",
        Rarity::Mythic => "Mythic",
        Rarity::Unique => "Unique",
    }
}
