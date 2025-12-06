-- Shadow OT Kill Statistics Migration
-- Adds kill records table for tracking deaths and kills

-- ============================================
-- KILL TYPE ENUM
-- ============================================

CREATE TYPE kill_type AS ENUM ('pvp', 'pve', 'boss');

-- ============================================
-- KILL RECORDS TABLE
-- ============================================

CREATE TABLE kill_records (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Victim information
    victim_id INTEGER REFERENCES characters(id) ON DELETE SET NULL,
    victim_name VARCHAR(64) NOT NULL, -- Cached for display
    victim_level INTEGER NOT NULL,
    victim_vocation VARCHAR(32) NOT NULL,
    
    -- Killer information
    killer_id INTEGER REFERENCES characters(id) ON DELETE SET NULL, -- NULL for creature kills
    killer_name VARCHAR(255) NOT NULL, -- Player name or creature name
    killer_level INTEGER, -- NULL for creatures
    killer_type VARCHAR(32) NOT NULL DEFAULT 'creature', -- 'player' or 'creature'
    
    -- Kill details
    kill_type kill_type NOT NULL DEFAULT 'pve',
    damage INTEGER NOT NULL DEFAULT 0,
    location VARCHAR(255) NOT NULL DEFAULT 'Unknown',
    realm VARCHAR(64) NOT NULL,
    
    -- Optional metadata
    weapon_used VARCHAR(128),
    spell_used VARCHAR(128),
    assists JSONB DEFAULT '[]', -- Array of {player_name, damage}
    
    -- Timestamps
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Partitioning support (by date)
    created_date DATE GENERATED ALWAYS AS (DATE(timestamp)) STORED
);

-- Indexes for common queries
CREATE INDEX idx_kill_records_victim ON kill_records(victim_id);
CREATE INDEX idx_kill_records_killer ON kill_records(killer_id) WHERE killer_type = 'player';
CREATE INDEX idx_kill_records_type ON kill_records(kill_type);
CREATE INDEX idx_kill_records_timestamp ON kill_records(timestamp DESC);
CREATE INDEX idx_kill_records_realm ON kill_records(realm);
CREATE INDEX idx_kill_records_location ON kill_records(location);
CREATE INDEX idx_kill_records_creature ON kill_records(killer_name) WHERE killer_type = 'creature';

-- Composite index for time-based queries
CREATE INDEX idx_kill_records_realm_time ON kill_records(realm, timestamp DESC);

-- ============================================
-- CHARACTER KILL STATISTICS (Cached Summary)
-- ============================================

CREATE TABLE character_kill_stats (
    character_id INTEGER PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
    
    -- Kill counts
    total_kills INTEGER NOT NULL DEFAULT 0,
    pvp_kills INTEGER NOT NULL DEFAULT 0,
    pve_kills INTEGER NOT NULL DEFAULT 0,
    boss_kills INTEGER NOT NULL DEFAULT 0,
    
    -- Death counts
    total_deaths INTEGER NOT NULL DEFAULT 0,
    pvp_deaths INTEGER NOT NULL DEFAULT 0,
    pve_deaths INTEGER NOT NULL DEFAULT 0,
    
    -- Streaks
    kill_streak INTEGER NOT NULL DEFAULT 0,
    best_kill_streak INTEGER NOT NULL DEFAULT 0,
    death_streak INTEGER NOT NULL DEFAULT 0,
    
    -- Unique kills
    unique_players_killed INTEGER NOT NULL DEFAULT 0,
    unique_creatures_killed INTEGER NOT NULL DEFAULT 0,
    unique_bosses_killed INTEGER NOT NULL DEFAULT 0,
    
    -- Last activity
    last_kill_at TIMESTAMP WITH TIME ZONE,
    last_death_at TIMESTAMP WITH TIME ZONE,
    
    -- Timestamps
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ============================================
-- CREATURE KILL STATS (Server-wide)
-- ============================================

CREATE TABLE creature_kill_stats (
    creature_name VARCHAR(255) NOT NULL,
    realm VARCHAR(64) NOT NULL,
    
    -- Counts
    total_kills INTEGER NOT NULL DEFAULT 0,
    kills_today INTEGER NOT NULL DEFAULT 0,
    kills_this_week INTEGER NOT NULL DEFAULT 0,
    kills_this_month INTEGER NOT NULL DEFAULT 0,
    
    -- Player counts
    killed_by_knights INTEGER NOT NULL DEFAULT 0,
    killed_by_paladins INTEGER NOT NULL DEFAULT 0,
    killed_by_sorcerers INTEGER NOT NULL DEFAULT 0,
    killed_by_druids INTEGER NOT NULL DEFAULT 0,
    
    -- Records
    fastest_kill_time INTEGER, -- milliseconds
    fastest_killer VARCHAR(64),
    highest_damage_kill INTEGER,
    
    -- Timestamps
    last_killed_at TIMESTAMP WITH TIME ZONE,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY (creature_name, realm)
);

CREATE INDEX idx_creature_kill_stats_kills ON creature_kill_stats(total_kills DESC);

-- ============================================
-- BOSS KILL RECORDS (Special tracking)
-- ============================================

CREATE TABLE boss_kills (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    boss_name VARCHAR(255) NOT NULL,
    realm VARCHAR(64) NOT NULL,
    
    -- Kill participants
    killer_id INTEGER REFERENCES characters(id) ON DELETE SET NULL,
    killer_name VARCHAR(64) NOT NULL,
    party_members JSONB DEFAULT '[]', -- [{name, damage, level}]
    
    -- Kill details
    kill_time INTEGER, -- milliseconds
    total_damage INTEGER,
    loot_dropped JSONB DEFAULT '[]', -- [{item_id, count}]
    
    -- Timestamps
    killed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_boss_kills_boss ON boss_kills(boss_name, realm);
CREATE INDEX idx_boss_kills_killer ON boss_kills(killer_id);
CREATE INDEX idx_boss_kills_time ON boss_kills(killed_at DESC);

-- ============================================
-- FUNCTIONS FOR UPDATING STATS
-- ============================================

-- Function to update character stats on kill
CREATE OR REPLACE FUNCTION update_character_kill_stats()
RETURNS TRIGGER AS $$
BEGIN
    -- Update killer stats (if player)
    IF NEW.killer_type = 'player' AND NEW.killer_id IS NOT NULL THEN
        INSERT INTO character_kill_stats (character_id, total_kills, pvp_kills, pve_kills, boss_kills, kill_streak, best_kill_streak, last_kill_at)
        VALUES (
            NEW.killer_id,
            1,
            CASE WHEN NEW.kill_type = 'pvp' THEN 1 ELSE 0 END,
            CASE WHEN NEW.kill_type = 'pve' THEN 1 ELSE 0 END,
            CASE WHEN NEW.kill_type = 'boss' THEN 1 ELSE 0 END,
            1,
            1,
            NEW.timestamp
        )
        ON CONFLICT (character_id) DO UPDATE SET
            total_kills = character_kill_stats.total_kills + 1,
            pvp_kills = character_kill_stats.pvp_kills + CASE WHEN NEW.kill_type = 'pvp' THEN 1 ELSE 0 END,
            pve_kills = character_kill_stats.pve_kills + CASE WHEN NEW.kill_type = 'pve' THEN 1 ELSE 0 END,
            boss_kills = character_kill_stats.boss_kills + CASE WHEN NEW.kill_type = 'boss' THEN 1 ELSE 0 END,
            kill_streak = character_kill_stats.kill_streak + 1,
            best_kill_streak = GREATEST(character_kill_stats.best_kill_streak, character_kill_stats.kill_streak + 1),
            death_streak = 0,
            last_kill_at = NEW.timestamp,
            updated_at = CURRENT_TIMESTAMP;
    END IF;
    
    -- Update victim stats
    IF NEW.victim_id IS NOT NULL THEN
        INSERT INTO character_kill_stats (character_id, total_deaths, pvp_deaths, pve_deaths, death_streak, last_death_at)
        VALUES (
            NEW.victim_id,
            1,
            CASE WHEN NEW.kill_type = 'pvp' THEN 1 ELSE 0 END,
            CASE WHEN NEW.kill_type != 'pvp' THEN 1 ELSE 0 END,
            1,
            NEW.timestamp
        )
        ON CONFLICT (character_id) DO UPDATE SET
            total_deaths = character_kill_stats.total_deaths + 1,
            pvp_deaths = character_kill_stats.pvp_deaths + CASE WHEN NEW.kill_type = 'pvp' THEN 1 ELSE 0 END,
            pve_deaths = character_kill_stats.pve_deaths + CASE WHEN NEW.kill_type != 'pvp' THEN 1 ELSE 0 END,
            kill_streak = 0,
            death_streak = character_kill_stats.death_streak + 1,
            last_death_at = NEW.timestamp,
            updated_at = CURRENT_TIMESTAMP;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_kill_stats
    AFTER INSERT ON kill_records
    FOR EACH ROW
    EXECUTE FUNCTION update_character_kill_stats();

-- Function to update creature stats
CREATE OR REPLACE FUNCTION update_creature_kill_stats()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.killer_type = 'creature' THEN
        -- This is a PvE death, update creature stats
        INSERT INTO creature_kill_stats (creature_name, realm, total_kills, kills_today, kills_this_week, kills_this_month, last_killed_at)
        VALUES (NEW.killer_name, NEW.realm, 1, 1, 1, 1, NEW.timestamp)
        ON CONFLICT (creature_name, realm) DO UPDATE SET
            total_kills = creature_kill_stats.total_kills + 1,
            kills_today = CASE 
                WHEN DATE(creature_kill_stats.last_killed_at) = DATE(NEW.timestamp) 
                THEN creature_kill_stats.kills_today + 1 
                ELSE 1 
            END,
            kills_this_week = CASE 
                WHEN DATE_TRUNC('week', creature_kill_stats.last_killed_at) = DATE_TRUNC('week', NEW.timestamp) 
                THEN creature_kill_stats.kills_this_week + 1 
                ELSE 1 
            END,
            kills_this_month = CASE 
                WHEN DATE_TRUNC('month', creature_kill_stats.last_killed_at) = DATE_TRUNC('month', NEW.timestamp) 
                THEN creature_kill_stats.kills_this_month + 1 
                ELSE 1 
            END,
            last_killed_at = NEW.timestamp,
            updated_at = CURRENT_TIMESTAMP;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_creature_stats
    AFTER INSERT ON kill_records
    FOR EACH ROW
    EXECUTE FUNCTION update_creature_kill_stats();

-- ============================================
-- VIEWS FOR COMMON QUERIES
-- ============================================

-- View for recent PvP kills
CREATE VIEW recent_pvp_kills AS
SELECT 
    kr.id,
    kr.victim_name,
    kr.victim_level,
    kr.killer_name,
    kr.killer_level,
    kr.damage,
    kr.location,
    kr.realm,
    kr.timestamp
FROM kill_records kr
WHERE kr.kill_type = 'pvp'
ORDER BY kr.timestamp DESC
LIMIT 100;

-- View for today's top killers
CREATE VIEW todays_top_killers AS
SELECT 
    kr.killer_id,
    kr.killer_name,
    COUNT(*) as kills,
    kr.realm
FROM kill_records kr
WHERE kr.killer_type = 'player'
  AND kr.timestamp >= CURRENT_DATE
GROUP BY kr.killer_id, kr.killer_name, kr.realm
ORDER BY kills DESC
LIMIT 50;
