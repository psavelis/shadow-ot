-- Migration: 006_spells_and_events.sql
-- Shadow OT - Spells, Runes, and Game Events tables

-- Spell element and type enums
CREATE TYPE spell_element AS ENUM ('fire', 'ice', 'earth', 'energy', 'holy', 'death', 'physical', 'healing');
CREATE TYPE spell_type AS ENUM ('instant', 'rune', 'conjure', 'support', 'special');

-- Spells table
CREATE TABLE IF NOT EXISTS spells (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    words VARCHAR(50) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    spell_type spell_type NOT NULL DEFAULT 'instant',
    element spell_element NOT NULL DEFAULT 'physical',
    level_required INTEGER NOT NULL DEFAULT 1,
    mana_cost INTEGER NOT NULL DEFAULT 0,
    soul_cost INTEGER NOT NULL DEFAULT 0,
    cooldown INTEGER NOT NULL DEFAULT 2000,
    group_cooldown INTEGER NOT NULL DEFAULT 2000,
    premium BOOLEAN NOT NULL DEFAULT FALSE,
    damage_formula TEXT,
    healing_formula TEXT,
    area_effect TEXT,
    icon VARCHAR(255),
    animation_id INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Spell vocations (many-to-many)
CREATE TABLE IF NOT EXISTS spell_vocations (
    id SERIAL PRIMARY KEY,
    spell_id INTEGER NOT NULL REFERENCES spells(id) ON DELETE CASCADE,
    vocation VARCHAR(50) NOT NULL,
    UNIQUE(spell_id, vocation)
);

-- Runes table
CREATE TABLE IF NOT EXISTS runes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    spell_id INTEGER NOT NULL REFERENCES spells(id),
    level_required INTEGER NOT NULL DEFAULT 1,
    magic_level_required INTEGER NOT NULL DEFAULT 0,
    charges INTEGER NOT NULL DEFAULT 1,
    element spell_element NOT NULL DEFAULT 'physical',
    premium BOOLEAN NOT NULL DEFAULT FALSE,
    description TEXT NOT NULL DEFAULT '',
    icon VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Rune vocations (many-to-many)
CREATE TABLE IF NOT EXISTS rune_vocations (
    id SERIAL PRIMARY KEY,
    rune_id INTEGER NOT NULL REFERENCES runes(id) ON DELETE CASCADE,
    vocation VARCHAR(50) NOT NULL,
    UNIQUE(rune_id, vocation)
);

-- Event status and type enums
CREATE TYPE event_status AS ENUM ('upcoming', 'active', 'ended', 'cancelled');
CREATE TYPE event_type AS ENUM ('world_boss', 'double_exp', 'double_skill', 'double_loot', 'raid', 'competition', 'seasonal', 'special');

-- Game events table
CREATE TABLE IF NOT EXISTS game_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    description TEXT NOT NULL,
    event_type event_type NOT NULL DEFAULT 'special',
    status event_status NOT NULL DEFAULT 'upcoming',
    realm VARCHAR(50),
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    location_x INTEGER,
    location_y INTEGER,
    location_z INTEGER,
    location_name VARCHAR(100),
    min_level INTEGER,
    max_players INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Event rewards
CREATE TABLE IF NOT EXISTS event_rewards (
    id SERIAL PRIMARY KEY,
    event_id UUID NOT NULL REFERENCES game_events(id) ON DELETE CASCADE,
    reward_type VARCHAR(50) NOT NULL,
    item_id INTEGER REFERENCES items(id),
    amount INTEGER NOT NULL DEFAULT 1,
    description TEXT NOT NULL DEFAULT '',
    tier VARCHAR(20)
);

-- Indexes for spells
CREATE INDEX IF NOT EXISTS idx_spells_type ON spells(spell_type);
CREATE INDEX IF NOT EXISTS idx_spells_element ON spells(element);
CREATE INDEX IF NOT EXISTS idx_spells_level ON spells(level_required);
CREATE INDEX IF NOT EXISTS idx_spell_vocations_spell ON spell_vocations(spell_id);
CREATE INDEX IF NOT EXISTS idx_spell_vocations_voc ON spell_vocations(vocation);

-- Indexes for runes
CREATE INDEX IF NOT EXISTS idx_runes_spell ON runes(spell_id);
CREATE INDEX IF NOT EXISTS idx_runes_level ON runes(level_required);

-- Indexes for events
CREATE INDEX IF NOT EXISTS idx_game_events_status ON game_events(status);
CREATE INDEX IF NOT EXISTS idx_game_events_type ON game_events(event_type);
CREATE INDEX IF NOT EXISTS idx_game_events_realm ON game_events(realm);
CREATE INDEX IF NOT EXISTS idx_game_events_dates ON game_events(starts_at, ends_at);

-- Sample spells data
INSERT INTO spells (name, words, description, spell_type, element, level_required, mana_cost, cooldown, premium) VALUES
    ('Light Healing', 'exura', 'Heals a small amount of health', 'instant', 'healing', 8, 20, 1000, false),
    ('Intense Healing', 'exura gran', 'Heals a moderate amount of health', 'instant', 'healing', 20, 70, 1000, false),
    ('Ultimate Healing', 'exura vita', 'Heals a large amount of health', 'instant', 'healing', 30, 160, 1000, false),
    ('Haste', 'utani hur', 'Temporarily increases movement speed', 'support', 'physical', 14, 60, 2000, false),
    ('Strong Haste', 'utani gran hur', 'Greatly increases movement speed', 'support', 'physical', 20, 100, 2000, true),
    ('Magic Shield', 'utamo vita', 'Converts damage to mana drain', 'support', 'energy', 14, 50, 14000, false),
    ('Invisible', 'utana vid', 'Become invisible to creatures', 'support', 'holy', 35, 440, 2000, true),
    ('Fire Wave', 'exevo flam hur', 'Shoots a wave of fire', 'instant', 'fire', 18, 25, 4000, false),
    ('Energy Beam', 'exevo vis lux', 'Shoots a beam of energy', 'instant', 'energy', 23, 40, 4000, false),
    ('Great Fireball', 'adori gran flam', 'Creates a great fireball rune', 'rune', 'fire', 30, 530, 2000, false),
    ('Sudden Death', 'adori gran mort', 'Creates a sudden death rune', 'rune', 'death', 45, 985, 2000, true),
    ('Ultimate Light', 'utevo gran lux', 'Creates a bright light around you', 'instant', 'holy', 26, 140, 2000, false),
    ('Brutal Strike', 'exori ico', 'Delivers a powerful melee strike', 'instant', 'physical', 16, 30, 6000, true),
    ('Whirlwind Throw', 'exori hur', 'Throws a whirling weapon', 'instant', 'physical', 28, 40, 6000, true),
    ('Divine Healing', 'exura san', 'Heals using divine energy', 'instant', 'holy', 35, 160, 1000, false);

-- Add vocations to spells
INSERT INTO spell_vocations (spell_id, vocation) VALUES
    (1, 'druid'), (1, 'sorcerer'), (1, 'paladin'), (1, 'knight'),
    (2, 'druid'), (2, 'sorcerer'), (2, 'paladin'),
    (3, 'druid'), (3, 'sorcerer'),
    (4, 'all'),
    (5, 'druid'), (5, 'sorcerer'),
    (6, 'all'),
    (7, 'druid'), (7, 'sorcerer'),
    (8, 'sorcerer'),
    (9, 'sorcerer'),
    (10, 'sorcerer'),
    (11, 'sorcerer'),
    (12, 'druid'), (12, 'sorcerer'),
    (13, 'knight'),
    (14, 'knight'), (14, 'paladin'),
    (15, 'paladin');

-- Sample runes
INSERT INTO runes (name, spell_id, level_required, magic_level_required, charges, element, premium, description) VALUES
    ('Great Fireball Rune', 10, 30, 4, 4, 'fire', false, 'Shoots a fireball dealing area fire damage'),
    ('Sudden Death Rune', 11, 45, 15, 3, 'death', true, 'Shoots a deadly bolt dealing massive death damage');

INSERT INTO rune_vocations (rune_id, vocation) VALUES
    (1, 'all'),
    (2, 'all');

-- Sample events
INSERT INTO game_events (name, description, event_type, status, starts_at, ends_at, min_level) VALUES
    ('Double Experience Weekend', 'Gain double experience for all kills!', 'double_exp', 'upcoming', 
     CURRENT_TIMESTAMP + INTERVAL '7 days', CURRENT_TIMESTAMP + INTERVAL '9 days', NULL),
    ('World Boss: Ancient Dragon', 'A legendary dragon awakens in the depths!', 'world_boss', 'upcoming',
     CURRENT_TIMESTAMP + INTERVAL '3 days', CURRENT_TIMESTAMP + INTERVAL '3 days' + INTERVAL '2 hours', 150),
    ('Halloween Event', 'Spooky creatures invade the lands!', 'seasonal', 'upcoming',
     CURRENT_TIMESTAMP + INTERVAL '30 days', CURRENT_TIMESTAMP + INTERVAL '44 days', NULL);
