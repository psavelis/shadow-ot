//! Database models module
//!
//! All database entities and their relationships

pub mod account;
pub mod blockchain;
pub mod character;
pub mod forum;
pub mod guild;
pub mod house;
pub mod item;
pub mod market;
pub mod quest;
pub mod realm;
pub mod social;
pub mod stats;

// Re-export commonly used models
pub use account::{Account, AccountSession, AccountType};
pub use character::{Character, CharacterSkill, CharacterSpell, CharacterDeath, Vocation, Sex, SkullType, SkillType};
pub use guild::{Guild, GuildRank, GuildMember, GuildInvite};
pub use house::{House, HouseAccess, HouseBid, AccessLevel};
pub use market::{MarketOffer, MarketHistory, OfferType, OfferState};
pub use realm::{Realm, RealmType, RealmStatus};
