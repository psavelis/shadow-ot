-- Migration: 005_achievements_world_quests_inventory.sql
-- Shadow OT - Achievements, World Quests, and Inventory tables

-- Achievement categories and rarities
CREATE TYPE achievement_category AS ENUM ('exploration', 'combat', 'social', 'economy', 'collection', 'special');
CREATE TYPE achievement_rarity AS ENUM ('common', 'uncommon', 'rare', 'epic', 'legendary');

-- Achievements
CREATE TABLE IF NOT EXISTS achievements (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    category achievement_category NOT NULL DEFAULT 'special',
    points INTEGER NOT NULL DEFAULT 1,
    rarity achievement_rarity NOT NULL DEFAULT 'common',
    secret BOOLEAN NOT NULL DEFAULT FALSE,
    icon VARCHAR(255),
    requirements TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Character achievements
CREATE TABLE IF NOT EXISTS character_achievements (
    id SERIAL PRIMARY KEY,
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    achievement_id INTEGER NOT NULL REFERENCES achievements(id) ON DELETE CASCADE,
    unlocked_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(character_id, achievement_id)
);

-- Achievement progress tracking
CREATE TABLE IF NOT EXISTS character_achievement_progress (
    id SERIAL PRIMARY KEY,
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    achievement_id INTEGER NOT NULL REFERENCES achievements(id) ON DELETE CASCADE,
    current_progress INTEGER NOT NULL DEFAULT 0,
    required_progress INTEGER NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(character_id, achievement_id)
);

-- World quest status
CREATE TYPE world_quest_status AS ENUM ('active', 'completed', 'failed');

-- World quests
CREATE TABLE IF NOT EXISTS world_quests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    description TEXT NOT NULL,
    realm VARCHAR(50) NOT NULL,
    status world_quest_status NOT NULL DEFAULT 'active',
    required_progress BIGINT NOT NULL,
    current_progress BIGINT NOT NULL DEFAULT 0,
    contributor_count INTEGER NOT NULL DEFAULT 0,
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- World quest rewards
CREATE TABLE IF NOT EXISTS world_quest_rewards (
    id SERIAL PRIMARY KEY,
    quest_id UUID NOT NULL REFERENCES world_quests(id) ON DELETE CASCADE,
    reward_type VARCHAR(50) NOT NULL,
    item_id INTEGER REFERENCES items(id),
    amount INTEGER NOT NULL DEFAULT 1,
    description TEXT NOT NULL DEFAULT ''
);

-- World quest contributions
CREATE TABLE IF NOT EXISTS world_quest_contributions (
    id SERIAL PRIMARY KEY,
    quest_id UUID NOT NULL REFERENCES world_quests(id) ON DELETE CASCADE,
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    amount BIGINT NOT NULL DEFAULT 0,
    contributed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(quest_id, character_id)
);

-- Character inventory
CREATE TABLE IF NOT EXISTS character_inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL,
    item_id INTEGER NOT NULL REFERENCES items(id),
    count INTEGER NOT NULL DEFAULT 1,
    slot INTEGER NOT NULL DEFAULT 0,
    container_id UUID,
    attack INTEGER,
    defense INTEGER,
    armor INTEGER,
    charges INTEGER,
    duration INTEGER,
    acquired_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(uuid) ON DELETE CASCADE
);

-- Imbuements
CREATE TABLE IF NOT EXISTS imbuements (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    type VARCHAR(50) NOT NULL,
    effect JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Inventory imbuements
CREATE TABLE IF NOT EXISTS inventory_imbuements (
    id SERIAL PRIMARY KEY,
    inventory_id UUID NOT NULL REFERENCES character_inventory(id) ON DELETE CASCADE,
    imbuement_id INTEGER NOT NULL REFERENCES imbuements(id),
    tier INTEGER NOT NULL DEFAULT 1,
    remaining_hours REAL NOT NULL DEFAULT 20,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Item transfers log
CREATE TABLE IF NOT EXISTS item_transfers (
    id SERIAL PRIMARY KEY,
    item_id INTEGER NOT NULL REFERENCES items(id),
    from_character_id UUID NOT NULL,
    to_character_id UUID NOT NULL,
    count INTEGER NOT NULL DEFAULT 1,
    transferred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for achievements
CREATE INDEX IF NOT EXISTS idx_achievements_category ON achievements(category);
CREATE INDEX IF NOT EXISTS idx_achievements_rarity ON achievements(rarity);
CREATE INDEX IF NOT EXISTS idx_character_achievements_char ON character_achievements(character_id);
CREATE INDEX IF NOT EXISTS idx_character_achievements_ach ON character_achievements(achievement_id);

-- Indexes for world quests
CREATE INDEX IF NOT EXISTS idx_world_quests_realm ON world_quests(realm);
CREATE INDEX IF NOT EXISTS idx_world_quests_status ON world_quests(status);
CREATE INDEX IF NOT EXISTS idx_world_quests_dates ON world_quests(starts_at, ends_at);
CREATE INDEX IF NOT EXISTS idx_world_quest_contributions_quest ON world_quest_contributions(quest_id);

-- Indexes for inventory
CREATE INDEX IF NOT EXISTS idx_character_inventory_char ON character_inventory(character_id);
CREATE INDEX IF NOT EXISTS idx_character_inventory_item ON character_inventory(item_id);
CREATE INDEX IF NOT EXISTS idx_character_inventory_slot ON character_inventory(character_id, slot);
CREATE INDEX IF NOT EXISTS idx_item_transfers_from ON item_transfers(from_character_id);
CREATE INDEX IF NOT EXISTS idx_item_transfers_to ON item_transfers(to_character_id);

-- Sample achievements data
INSERT INTO achievements (name, description, category, points, rarity, secret) VALUES
    ('First Steps', 'Create your first character', 'exploration', 10, 'common', false),
    ('Explorer', 'Visit all major cities', 'exploration', 50, 'uncommon', false),
    ('World Traveler', 'Visit every realm', 'exploration', 100, 'rare', false),
    ('Dragon Slayer', 'Defeat a dragon', 'combat', 100, 'epic', false),
    ('Boss Hunter', 'Defeat 10 bosses', 'combat', 200, 'rare', false),
    ('Guild Founder', 'Create a guild', 'social', 50, 'uncommon', false),
    ('Wealthy', 'Accumulate 1 million gold', 'economy', 100, 'rare', false),
    ('Collector', 'Collect 100 unique items', 'collection', 150, 'rare', false),
    ('Hidden Treasure', 'Find the secret treasure', 'special', 500, 'legendary', true),
    ('Veteran', 'Play for over 1000 hours', 'special', 300, 'epic', false);

-- Sample imbuements
INSERT INTO imbuements (name, description, type, effect) VALUES
    ('Powerful Strike', 'Increases physical damage', 'attack', '{"damage_bonus": 25}'),
    ('Vampirism', 'Life leech effect', 'attack', '{"life_leech": 10}'),
    ('Mana Leech', 'Mana leech effect', 'attack', '{"mana_leech": 8}'),
    ('Critical Strike', 'Increases critical hit chance', 'attack', '{"crit_chance": 10}'),
    ('Protection', 'Reduces physical damage taken', 'defense', '{"damage_reduction": 15}'),
    ('Fire Resistance', 'Reduces fire damage taken', 'defense', '{"fire_resistance": 20}'),
    ('Ice Resistance', 'Reduces ice damage taken', 'defense', '{"ice_resistance": 20}'),
    ('Energy Resistance', 'Reduces energy damage taken', 'defense', '{"energy_resistance": 20}'),
    ('Speed', 'Increases movement speed', 'utility', '{"speed_bonus": 10}'),
    ('Capacity', 'Increases carrying capacity', 'utility', '{"capacity_bonus": 100}');
