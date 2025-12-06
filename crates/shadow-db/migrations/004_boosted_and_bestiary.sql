-- Shadow OT Boosted and Bestiary Migration
-- Adds creatures, boosted creatures/bosses, and bestiary tracking

-- ============================================
-- CREATURE DIFFICULTY ENUM
-- ============================================

CREATE TYPE creature_difficulty AS ENUM ('harmless', 'trivial', 'easy', 'medium', 'hard', 'challenging');

-- ============================================
-- CREATURES TABLE
-- ============================================

CREATE TABLE creatures (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    race VARCHAR(64) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    experience INTEGER NOT NULL DEFAULT 0,
    health INTEGER NOT NULL DEFAULT 1,
    speed INTEGER NOT NULL DEFAULT 100,
    armor INTEGER NOT NULL DEFAULT 0,
    difficulty creature_difficulty NOT NULL DEFAULT 'medium',
    is_boss BOOLEAN NOT NULL DEFAULT FALSE,
    sprite_id INTEGER NOT NULL DEFAULT 0,
    immunities JSONB NOT NULL DEFAULT '[]',
    weaknesses JSONB NOT NULL DEFAULT '[]',
    abilities JSONB NOT NULL DEFAULT '[]',
    locations JSONB NOT NULL DEFAULT '[]',
    charm_points INTEGER NOT NULL DEFAULT 0,
    bestiary_class VARCHAR(64) NOT NULL DEFAULT 'Misc',
    bestiary_occurrence VARCHAR(64) NOT NULL DEFAULT 'Common',
    creature_rarity INTEGER NOT NULL DEFAULT 0, -- For sorting by rarity
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_creatures_name ON creatures(name);
CREATE INDEX idx_creatures_race ON creatures(race);
CREATE INDEX idx_creatures_difficulty ON creatures(difficulty);
CREATE INDEX idx_creatures_is_boss ON creatures(is_boss) WHERE is_boss = TRUE;
CREATE INDEX idx_creatures_bestiary_class ON creatures(bestiary_class);

-- ============================================
-- CREATURE LOOT
-- ============================================

CREATE TABLE creature_loot (
    id SERIAL PRIMARY KEY,
    creature_id INTEGER REFERENCES creatures(id) ON DELETE CASCADE NOT NULL,
    item_id INTEGER NOT NULL,
    chance REAL NOT NULL DEFAULT 0.0, -- Percentage 0-100
    max_count INTEGER NOT NULL DEFAULT 1,
    
    UNIQUE(creature_id, item_id)
);

CREATE INDEX idx_creature_loot_creature ON creature_loot(creature_id);

-- ============================================
-- BOOSTED CREATURES (Daily Rotating)
-- ============================================

CREATE TABLE boosted_creatures (
    id SERIAL PRIMARY KEY,
    creature_id INTEGER REFERENCES creatures(id) ON DELETE CASCADE NOT NULL,
    date DATE NOT NULL,
    experience_bonus INTEGER NOT NULL DEFAULT 50, -- Percentage bonus
    loot_bonus INTEGER NOT NULL DEFAULT 50, -- Percentage bonus
    
    UNIQUE(date) -- Only one boosted creature per day
);

CREATE INDEX idx_boosted_creatures_date ON boosted_creatures(date DESC);

-- ============================================
-- BOOSTED BOSSES (Daily Rotating)
-- ============================================

CREATE TABLE boosted_bosses (
    id SERIAL PRIMARY KEY,
    boss_id INTEGER REFERENCES creatures(id) ON DELETE CASCADE NOT NULL,
    date DATE NOT NULL,
    experience_bonus INTEGER NOT NULL DEFAULT 100,
    loot_bonus INTEGER NOT NULL DEFAULT 100,
    
    UNIQUE(date)
);

CREATE INDEX idx_boosted_bosses_date ON boosted_bosses(date DESC);

-- ============================================
-- BESTIARY PROGRESS
-- ============================================

CREATE TABLE bestiary_progress (
    id SERIAL PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE NOT NULL,
    creature_id INTEGER REFERENCES creatures(id) ON DELETE CASCADE NOT NULL,
    kills INTEGER NOT NULL DEFAULT 0,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    unlocked_loot BOOLEAN NOT NULL DEFAULT FALSE,
    unlocked_charm BOOLEAN NOT NULL DEFAULT FALSE,
    first_kill_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(character_id, creature_id)
);

CREATE INDEX idx_bestiary_progress_character ON bestiary_progress(character_id);
CREATE INDEX idx_bestiary_progress_creature ON bestiary_progress(creature_id);
CREATE INDEX idx_bestiary_progress_completed ON bestiary_progress(character_id, completed) WHERE completed = TRUE;

-- ============================================
-- CHARMS (Bestiary Rewards)
-- ============================================

CREATE TABLE charms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) UNIQUE NOT NULL,
    description TEXT NOT NULL,
    effect_type VARCHAR(32) NOT NULL, -- 'damage', 'defense', 'utility'
    effect_value INTEGER NOT NULL DEFAULT 0,
    cost INTEGER NOT NULL DEFAULT 0, -- Charm points to unlock
    sprite_id INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE character_charms (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE NOT NULL,
    charm_id INTEGER REFERENCES charms(id) ON DELETE CASCADE NOT NULL,
    assigned_creature_id INTEGER REFERENCES creatures(id) ON DELETE SET NULL,
    unlocked_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY(character_id, charm_id)
);

CREATE INDEX idx_character_charms_character ON character_charms(character_id);

-- ============================================
-- FUNCTION: Automatically select daily boosted
-- ============================================

CREATE OR REPLACE FUNCTION select_daily_boosted()
RETURNS void AS $$
DECLARE
    random_creature_id INTEGER;
    random_boss_id INTEGER;
BEGIN
    -- Select random non-boss creature that hasn't been boosted recently
    SELECT id INTO random_creature_id
    FROM creatures
    WHERE is_boss = FALSE
      AND id NOT IN (
          SELECT creature_id FROM boosted_creatures
          WHERE date > CURRENT_DATE - INTERVAL '30 days'
      )
    ORDER BY RANDOM()
    LIMIT 1;
    
    -- Insert boosted creature for today
    IF random_creature_id IS NOT NULL THEN
        INSERT INTO boosted_creatures (creature_id, date, experience_bonus, loot_bonus)
        VALUES (random_creature_id, CURRENT_DATE, 50, 50)
        ON CONFLICT (date) DO NOTHING;
    END IF;
    
    -- Select random boss that hasn't been boosted recently
    SELECT id INTO random_boss_id
    FROM creatures
    WHERE is_boss = TRUE
      AND id NOT IN (
          SELECT boss_id FROM boosted_bosses
          WHERE date > CURRENT_DATE - INTERVAL '30 days'
      )
    ORDER BY RANDOM()
    LIMIT 1;
    
    -- Insert boosted boss for today
    IF random_boss_id IS NOT NULL THEN
        INSERT INTO boosted_bosses (boss_id, date, experience_bonus, loot_bonus)
        VALUES (random_boss_id, CURRENT_DATE, 100, 100)
        ON CONFLICT (date) DO NOTHING;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- FUNCTION: Update bestiary on creature kill
-- ============================================

CREATE OR REPLACE FUNCTION update_bestiary_on_kill(
    p_character_id INTEGER,
    p_creature_id INTEGER
)
RETURNS void AS $$
DECLARE
    current_kills INTEGER;
    occurrence TEXT;
    thresholds INTEGER[];
BEGIN
    -- Get creature occurrence for stage calculation
    SELECT bestiary_occurrence INTO occurrence
    FROM creatures WHERE id = p_creature_id;
    
    -- Determine thresholds based on occurrence
    thresholds := CASE LOWER(occurrence)
        WHEN 'common' THEN ARRAY[5, 250, 500, 1000]
        WHEN 'uncommon' THEN ARRAY[5, 100, 250, 500]
        WHEN 'rare' THEN ARRAY[1, 25, 50, 100]
        WHEN 'very rare' THEN ARRAY[1, 5, 10, 25]
        ELSE ARRAY[5, 250, 500, 1000]
    END;
    
    -- Upsert bestiary progress
    INSERT INTO bestiary_progress (character_id, creature_id, kills, first_kill_at)
    VALUES (p_character_id, p_creature_id, 1, CURRENT_TIMESTAMP)
    ON CONFLICT (character_id, creature_id) DO UPDATE SET
        kills = bestiary_progress.kills + 1,
        unlocked_loot = bestiary_progress.kills + 1 >= thresholds[2],
        unlocked_charm = bestiary_progress.kills + 1 >= thresholds[3],
        completed = bestiary_progress.kills + 1 >= thresholds[4],
        completed_at = CASE 
            WHEN bestiary_progress.kills + 1 >= thresholds[4] AND bestiary_progress.completed_at IS NULL 
            THEN CURRENT_TIMESTAMP 
            ELSE bestiary_progress.completed_at 
        END,
        updated_at = CURRENT_TIMESTAMP
    RETURNING kills INTO current_kills;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- SAMPLE DATA: Insert default charms
-- ============================================

INSERT INTO charms (name, description, effect_type, effect_value, cost) VALUES
    ('Wound', 'Triggers on the target with a chance of 10%. Deals 5% of its initial hit points as Physical damage once', 'damage', 5, 600),
    ('Enflame', 'Triggers on the target with a chance of 10%. Deals 5% of its initial hit points as Fire damage once', 'damage', 5, 1000),
    ('Poison', 'Triggers on the target with a chance of 10%. Deals 5% of its initial hit points as Earth damage once', 'damage', 5, 600),
    ('Freeze', 'Triggers on the target with a chance of 10%. Deals 5% of its initial hit points as Ice damage once', 'damage', 5, 800),
    ('Zap', 'Triggers on the target with a chance of 10%. Deals 5% of its initial hit points as Energy damage once', 'damage', 5, 800),
    ('Curse', 'Triggers on the target with a chance of 10%. Deals 5% of its initial hit points as Death damage once', 'damage', 5, 900),
    ('Divine Wrath', 'Triggers on the target with a chance of 10%. Deals 5% of its initial hit points as Holy damage once', 'damage', 5, 1500),
    ('Parry', 'Triggers with a chance of 10% every time you get hit. Redirects the damage back to attacker', 'defense', 10, 1500),
    ('Dodge', 'Triggers with a chance of 10% every time you get hit. Completely avoids the damage', 'defense', 10, 600),
    ('Adrenaline Burst', 'Triggers with a chance of 10%. Increases your movement speed for 10 seconds', 'utility', 10, 500),
    ('Numb', 'Reduces physical damage received from the creature by 10%', 'defense', 10, 500),
    ('Cleanse', 'Triggers with a chance of 10%. Removes one negative effect', 'utility', 10, 700),
    ('Bless', 'Increases experience gained from the creature by 8%', 'utility', 8, 2000),
    ('Scavenge', 'Increases loot chance from the creature by 25%', 'utility', 25, 1500),
    ('Gut', 'Increases chance to get creature products by 20%', 'utility', 20, 800),
    ('Low Blow', 'Increases critical hit chance against the creature by 8%', 'damage', 8, 2000),
    ('Cripple', 'Triggers with chance of 10%. Slows the creature by 20% for 5 seconds', 'utility', 20, 500);
