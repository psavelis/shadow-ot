-- Shadow OT Initial Database Schema
-- Core tables for accounts, characters, and game data

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================
-- ACCOUNTS AND AUTHENTICATION
-- ============================================

CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    uuid UUID DEFAULT uuid_generate_v4() UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    salt VARCHAR(64) NOT NULL,
    premium_until TIMESTAMP,
    coins INTEGER DEFAULT 0,
    tournament_coins INTEGER DEFAULT 0,
    creation_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP,
    email_verified BOOLEAN DEFAULT FALSE,
    email_verified_at TIMESTAMP,
    two_factor_enabled BOOLEAN DEFAULT FALSE,
    two_factor_secret VARCHAR(64),
    status VARCHAR(32) DEFAULT 'active',
    type VARCHAR(32) DEFAULT 'normal',
    auth_token VARCHAR(255),
    auth_token_expires TIMESTAMP,
    recovery_key VARCHAR(64),
    recovery_key_expires TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_accounts_email ON accounts(email);
CREATE INDEX idx_accounts_uuid ON accounts(uuid);
CREATE INDEX idx_accounts_status ON accounts(status);

CREATE TABLE account_sessions (
    id SERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    ip_address INET NOT NULL,
    user_agent TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL,
    last_activity TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    revoked BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_sessions_account ON account_sessions(account_id);
CREATE INDEX idx_sessions_token ON account_sessions(session_token);

CREATE TABLE account_bans (
    id SERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE,
    banned_by INTEGER REFERENCES accounts(id),
    reason TEXT NOT NULL,
    ban_type VARCHAR(32) DEFAULT 'temporary',
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    appealed BOOLEAN DEFAULT FALSE,
    appeal_text TEXT,
    appeal_resolved BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_bans_account ON account_bans(account_id);

CREATE TABLE account_auth_logs (
    id SERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE,
    action VARCHAR(64) NOT NULL,
    ip_address INET NOT NULL,
    user_agent TEXT,
    success BOOLEAN NOT NULL,
    details JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_auth_logs_account ON account_auth_logs(account_id);
CREATE INDEX idx_auth_logs_time ON account_auth_logs(created_at DESC);

-- ============================================
-- REALMS
-- ============================================

CREATE TABLE realms (
    id SERIAL PRIMARY KEY,
    uuid UUID DEFAULT uuid_generate_v4() UNIQUE NOT NULL,
    name VARCHAR(64) UNIQUE NOT NULL,
    slug VARCHAR(64) UNIQUE NOT NULL,
    description TEXT,
    theme VARCHAR(32) DEFAULT 'classic',
    pvp_type VARCHAR(32) DEFAULT 'open',
    game_type VARCHAR(32) DEFAULT 'pvp',
    region VARCHAR(32) DEFAULT 'us-east',
    status VARCHAR(32) DEFAULT 'online',
    ip_address INET,
    port INTEGER DEFAULT 7172,
    max_players INTEGER DEFAULT 1000,
    current_players INTEGER DEFAULT 0,
    creation_date DATE DEFAULT CURRENT_DATE,
    experience_rate DECIMAL(10,2) DEFAULT 1.0,
    skill_rate DECIMAL(10,2) DEFAULT 1.0,
    magic_rate DECIMAL(10,2) DEFAULT 1.0,
    loot_rate DECIMAL(10,2) DEFAULT 1.0,
    spawn_rate DECIMAL(10,2) DEFAULT 1.0,
    premium_only BOOLEAN DEFAULT FALSE,
    transfer_locked BOOLEAN DEFAULT FALSE,
    features JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_realms_status ON realms(status);
CREATE INDEX idx_realms_region ON realms(region);

-- ============================================
-- CHARACTERS
-- ============================================

CREATE TABLE characters (
    id SERIAL PRIMARY KEY,
    uuid UUID DEFAULT uuid_generate_v4() UNIQUE NOT NULL,
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE,
    realm_id INTEGER REFERENCES realms(id) ON DELETE CASCADE,
    name VARCHAR(32) UNIQUE NOT NULL,
    sex SMALLINT DEFAULT 1,
    vocation SMALLINT DEFAULT 0,
    level INTEGER DEFAULT 1,
    experience BIGINT DEFAULT 0,
    health INTEGER DEFAULT 150,
    max_health INTEGER DEFAULT 150,
    mana INTEGER DEFAULT 0,
    max_mana INTEGER DEFAULT 0,
    soul SMALLINT DEFAULT 100,
    stamina INTEGER DEFAULT 2520,
    look_type INTEGER DEFAULT 128,
    look_head SMALLINT DEFAULT 78,
    look_body SMALLINT DEFAULT 69,
    look_legs SMALLINT DEFAULT 58,
    look_feet SMALLINT DEFAULT 76,
    look_addons SMALLINT DEFAULT 0,
    look_mount INTEGER DEFAULT 0,
    town_id INTEGER DEFAULT 1,
    pos_x INTEGER DEFAULT 100,
    pos_y INTEGER DEFAULT 100,
    pos_z SMALLINT DEFAULT 7,
    cap INTEGER DEFAULT 400,
    balance BIGINT DEFAULT 0,
    bank_balance BIGINT DEFAULT 0,
    last_login TIMESTAMP,
    last_logout TIMESTAMP,
    online BOOLEAN DEFAULT FALSE,
    skill_fist INTEGER DEFAULT 10,
    skill_fist_tries BIGINT DEFAULT 0,
    skill_club INTEGER DEFAULT 10,
    skill_club_tries BIGINT DEFAULT 0,
    skill_sword INTEGER DEFAULT 10,
    skill_sword_tries BIGINT DEFAULT 0,
    skill_axe INTEGER DEFAULT 10,
    skill_axe_tries BIGINT DEFAULT 0,
    skill_dist INTEGER DEFAULT 10,
    skill_dist_tries BIGINT DEFAULT 0,
    skill_shielding INTEGER DEFAULT 10,
    skill_shielding_tries BIGINT DEFAULT 0,
    skill_fishing INTEGER DEFAULT 10,
    skill_fishing_tries BIGINT DEFAULT 0,
    magic_level INTEGER DEFAULT 0,
    mana_spent BIGINT DEFAULT 0,
    conditions BYTEA,
    skull SMALLINT DEFAULT 0,
    skull_time TIMESTAMP,
    unjustified_kills INTEGER DEFAULT 0,
    blessings INTEGER DEFAULT 0,
    deletion_time TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_characters_account ON characters(account_id);
CREATE INDEX idx_characters_realm ON characters(realm_id);
CREATE INDEX idx_characters_name ON characters(name);
CREATE INDEX idx_characters_level ON characters(level DESC);

CREATE TABLE character_deaths (
    id SERIAL PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    level INTEGER NOT NULL,
    killed_by VARCHAR(64) NOT NULL,
    is_player BOOLEAN DEFAULT FALSE,
    mostdamage_by VARCHAR(64),
    mostdamage_is_player BOOLEAN DEFAULT FALSE,
    unjustified BOOLEAN DEFAULT FALSE,
    death_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_deaths_character ON character_deaths(character_id);
CREATE INDEX idx_deaths_time ON character_deaths(death_time DESC);

CREATE TABLE character_storage (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    key INTEGER NOT NULL,
    value BIGINT NOT NULL,
    PRIMARY KEY (character_id, key)
);

CREATE TABLE character_spells (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    spell_name VARCHAR(64) NOT NULL,
    learned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (character_id, spell_name)
);

CREATE TABLE character_outfits (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    outfit_id INTEGER NOT NULL,
    addons SMALLINT DEFAULT 0,
    unlocked_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (character_id, outfit_id)
);

CREATE TABLE character_mounts (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    mount_id INTEGER NOT NULL,
    unlocked_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (character_id, mount_id)
);

-- Bestiary tracking
CREATE TABLE character_bestiary (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    race_id INTEGER NOT NULL,
    kill_count INTEGER DEFAULT 0,
    first_unlock BOOLEAN DEFAULT FALSE,
    second_unlock BOOLEAN DEFAULT FALSE,
    third_unlock BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (character_id, race_id)
);

-- Prey system
CREATE TABLE character_prey (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    slot_id SMALLINT NOT NULL,
    race_id INTEGER,
    bonus_type SMALLINT,
    bonus_value SMALLINT,
    bonus_grade SMALLINT,
    time_left INTEGER DEFAULT 0,
    free_reroll_time TIMESTAMP,
    PRIMARY KEY (character_id, slot_id)
);

-- ============================================
-- ITEMS AND CONTAINERS
-- ============================================

CREATE TABLE player_items (
    id SERIAL PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    pid INTEGER NOT NULL,
    sid INTEGER NOT NULL,
    itemtype INTEGER NOT NULL,
    count INTEGER DEFAULT 1,
    attributes BYTEA
);

CREATE INDEX idx_player_items_character ON player_items(character_id);

CREATE TABLE player_depot_items (
    id SERIAL PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    town_id INTEGER NOT NULL,
    pid INTEGER NOT NULL,
    sid INTEGER NOT NULL,
    itemtype INTEGER NOT NULL,
    count INTEGER DEFAULT 1,
    attributes BYTEA
);

CREATE INDEX idx_depot_items_character ON player_depot_items(character_id);

CREATE TABLE player_inbox (
    id SERIAL PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    pid INTEGER NOT NULL,
    sid INTEGER NOT NULL,
    itemtype INTEGER NOT NULL,
    count INTEGER DEFAULT 1,
    attributes BYTEA,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_inbox_character ON player_inbox(character_id);

-- ============================================
-- GUILDS
-- ============================================

CREATE TABLE guilds (
    id SERIAL PRIMARY KEY,
    uuid UUID DEFAULT uuid_generate_v4() UNIQUE NOT NULL,
    realm_id INTEGER REFERENCES realms(id) ON DELETE CASCADE,
    name VARCHAR(64) UNIQUE NOT NULL,
    owner_id INTEGER REFERENCES characters(id) ON DELETE SET NULL,
    motd TEXT,
    description TEXT,
    logo_url VARCHAR(512),
    balance BIGINT DEFAULT 0,
    level INTEGER DEFAULT 1,
    experience BIGINT DEFAULT 0,
    creation_date DATE DEFAULT CURRENT_DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_guilds_realm ON guilds(realm_id);
CREATE INDEX idx_guilds_owner ON guilds(owner_id);

CREATE TABLE guild_ranks (
    id SERIAL PRIMARY KEY,
    guild_id INTEGER REFERENCES guilds(id) ON DELETE CASCADE,
    name VARCHAR(32) NOT NULL,
    level INTEGER NOT NULL,
    permissions BIGINT DEFAULT 0,
    UNIQUE(guild_id, name)
);

CREATE TABLE guild_members (
    guild_id INTEGER REFERENCES guilds(id) ON DELETE CASCADE,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    rank_id INTEGER REFERENCES guild_ranks(id) ON DELETE SET NULL,
    nick VARCHAR(32),
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (guild_id, character_id)
);

CREATE INDEX idx_guild_members_character ON guild_members(character_id);

CREATE TABLE guild_invites (
    guild_id INTEGER REFERENCES guilds(id) ON DELETE CASCADE,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    invited_by INTEGER REFERENCES characters(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (guild_id, character_id)
);

CREATE TABLE guild_wars (
    id SERIAL PRIMARY KEY,
    guild1_id INTEGER REFERENCES guilds(id) ON DELETE CASCADE,
    guild2_id INTEGER REFERENCES guilds(id) ON DELETE CASCADE,
    status VARCHAR(32) DEFAULT 'pending',
    frag_limit INTEGER DEFAULT 0,
    kill_limit INTEGER DEFAULT 100,
    guild1_frags INTEGER DEFAULT 0,
    guild2_frags INTEGER DEFAULT 0,
    started_at TIMESTAMP,
    ended_at TIMESTAMP,
    winner_id INTEGER REFERENCES guilds(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE guild_war_kills (
    id SERIAL PRIMARY KEY,
    war_id INTEGER REFERENCES guild_wars(id) ON DELETE CASCADE,
    killer_id INTEGER REFERENCES characters(id),
    victim_id INTEGER REFERENCES characters(id),
    killed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ============================================
-- HOUSES
-- ============================================

CREATE TABLE houses (
    id SERIAL PRIMARY KEY,
    realm_id INTEGER REFERENCES realms(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    owner_id INTEGER REFERENCES characters(id) ON DELETE SET NULL,
    town_id INTEGER NOT NULL,
    rent INTEGER DEFAULT 0,
    size INTEGER DEFAULT 0,
    beds INTEGER DEFAULT 0,
    entry_x INTEGER NOT NULL,
    entry_y INTEGER NOT NULL,
    entry_z SMALLINT NOT NULL,
    paid_until TIMESTAMP,
    last_transfer TIMESTAMP,
    guild_id INTEGER REFERENCES guilds(id) ON DELETE SET NULL,
    UNIQUE(realm_id, name)
);

CREATE INDEX idx_houses_realm ON houses(realm_id);
CREATE INDEX idx_houses_owner ON houses(owner_id);
CREATE INDEX idx_houses_town ON houses(town_id);

CREATE TABLE house_lists (
    house_id INTEGER REFERENCES houses(id) ON DELETE CASCADE,
    list_id INTEGER NOT NULL,
    list_text TEXT,
    PRIMARY KEY (house_id, list_id)
);

CREATE TABLE house_bids (
    id SERIAL PRIMARY KEY,
    house_id INTEGER REFERENCES houses(id) ON DELETE CASCADE,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    bid_amount BIGINT NOT NULL,
    bid_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(house_id, character_id)
);

-- ============================================
-- MARKET
-- ============================================

CREATE TABLE market_offers (
    id SERIAL PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    realm_id INTEGER REFERENCES realms(id) ON DELETE CASCADE,
    item_type INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    price BIGINT NOT NULL,
    offer_type VARCHAR(8) NOT NULL, -- 'buy' or 'sell'
    anonymous BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL
);

CREATE INDEX idx_market_realm ON market_offers(realm_id);
CREATE INDEX idx_market_type ON market_offers(item_type);
CREATE INDEX idx_market_offer_type ON market_offers(offer_type);
CREATE INDEX idx_market_expires ON market_offers(expires_at);

CREATE TABLE market_history (
    id SERIAL PRIMARY KEY,
    realm_id INTEGER REFERENCES realms(id) ON DELETE CASCADE,
    buyer_id INTEGER REFERENCES characters(id),
    seller_id INTEGER REFERENCES characters(id),
    item_type INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    price BIGINT NOT NULL,
    completed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_market_history_realm ON market_history(realm_id);
CREATE INDEX idx_market_history_item ON market_history(item_type);

-- ============================================
-- QUESTS
-- ============================================

CREATE TABLE quests (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) UNIQUE NOT NULL,
    storage_key INTEGER NOT NULL,
    storage_value INTEGER NOT NULL,
    description TEXT
);

CREATE TABLE quest_missions (
    id SERIAL PRIMARY KEY,
    quest_id INTEGER REFERENCES quests(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    storage_key INTEGER NOT NULL,
    storage_value INTEGER NOT NULL,
    description TEXT
);

CREATE TABLE character_quests (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    quest_id INTEGER REFERENCES quests(id) ON DELETE CASCADE,
    completed BOOLEAN DEFAULT FALSE,
    progress JSONB DEFAULT '{}',
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    PRIMARY KEY (character_id, quest_id)
);

-- ============================================
-- SOCIAL
-- ============================================

CREATE TABLE vip_entries (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    vip_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    description VARCHAR(128),
    icon SMALLINT DEFAULT 0,
    notify BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (character_id, vip_id)
);

CREATE TABLE player_letters (
    id SERIAL PRIMARY KEY,
    sender_id INTEGER REFERENCES characters(id) ON DELETE SET NULL,
    recipient_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    subject VARCHAR(128),
    body TEXT,
    read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_letters_recipient ON player_letters(recipient_id);

-- ============================================
-- FORUM
-- ============================================

CREATE TABLE forum_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    description TEXT,
    position INTEGER DEFAULT 0,
    is_locked BOOLEAN DEFAULT FALSE,
    access_level VARCHAR(32) DEFAULT 'public'
);

CREATE TABLE forum_threads (
    id SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES forum_categories(id) ON DELETE CASCADE,
    author_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
    character_name VARCHAR(32),
    title VARCHAR(256) NOT NULL,
    is_sticky BOOLEAN DEFAULT FALSE,
    is_locked BOOLEAN DEFAULT FALSE,
    view_count INTEGER DEFAULT 0,
    reply_count INTEGER DEFAULT 0,
    last_reply_at TIMESTAMP,
    last_reply_by VARCHAR(32),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_threads_category ON forum_threads(category_id);
CREATE INDEX idx_threads_author ON forum_threads(author_id);
CREATE INDEX idx_threads_sticky ON forum_threads(is_sticky DESC);

CREATE TABLE forum_posts (
    id SERIAL PRIMARY KEY,
    thread_id INTEGER REFERENCES forum_threads(id) ON DELETE CASCADE,
    author_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
    character_name VARCHAR(32),
    content TEXT NOT NULL,
    is_first_post BOOLEAN DEFAULT FALSE,
    edited_by VARCHAR(32),
    edited_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_posts_thread ON forum_posts(thread_id);
CREATE INDEX idx_posts_author ON forum_posts(author_id);

-- ============================================
-- NEWS AND ANNOUNCEMENTS
-- ============================================

CREATE TABLE news_articles (
    id SERIAL PRIMARY KEY,
    author_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
    title VARCHAR(256) NOT NULL,
    content TEXT NOT NULL,
    category VARCHAR(32) DEFAULT 'news',
    is_published BOOLEAN DEFAULT FALSE,
    featured BOOLEAN DEFAULT FALSE,
    view_count INTEGER DEFAULT 0,
    published_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_news_published ON news_articles(is_published, published_at DESC);

-- ============================================
-- STATISTICS AND LOGS
-- ============================================

CREATE TABLE daily_stats (
    id SERIAL PRIMARY KEY,
    realm_id INTEGER REFERENCES realms(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    peak_players INTEGER DEFAULT 0,
    total_logins INTEGER DEFAULT 0,
    total_deaths INTEGER DEFAULT 0,
    total_experience BIGINT DEFAULT 0,
    total_loot BIGINT DEFAULT 0,
    monsters_killed BIGINT DEFAULT 0,
    players_killed INTEGER DEFAULT 0,
    UNIQUE(realm_id, date)
);

CREATE INDEX idx_stats_realm_date ON daily_stats(realm_id, date DESC);

CREATE TABLE player_online_history (
    id SERIAL PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    login_time TIMESTAMP NOT NULL,
    logout_time TIMESTAMP,
    ip_address INET,
    duration INTEGER
);

CREATE INDEX idx_online_character ON player_online_history(character_id);
CREATE INDEX idx_online_time ON player_online_history(login_time DESC);

-- ============================================
-- BLOCKCHAIN AND NFT
-- ============================================

CREATE TABLE wallet_links (
    id SERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE,
    chain VARCHAR(32) NOT NULL,
    address VARCHAR(128) NOT NULL,
    verified BOOLEAN DEFAULT FALSE,
    verification_message VARCHAR(512),
    verification_signature VARCHAR(512),
    is_primary BOOLEAN DEFAULT FALSE,
    linked_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(chain, address)
);

CREATE INDEX idx_wallets_account ON wallet_links(account_id);

CREATE TABLE nft_assets (
    id SERIAL PRIMARY KEY,
    uuid UUID DEFAULT uuid_generate_v4() UNIQUE NOT NULL,
    owner_account_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
    asset_type VARCHAR(32) NOT NULL,
    chain VARCHAR(32) NOT NULL,
    contract_address VARCHAR(128),
    token_id VARCHAR(128),
    metadata JSONB DEFAULT '{}',
    game_item_id INTEGER,
    game_character_id INTEGER REFERENCES characters(id) ON DELETE SET NULL,
    minted BOOLEAN DEFAULT FALSE,
    minted_at TIMESTAMP,
    mint_tx_hash VARCHAR(128),
    burned BOOLEAN DEFAULT FALSE,
    burned_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_nft_owner ON nft_assets(owner_account_id);
CREATE INDEX idx_nft_chain ON nft_assets(chain);

CREATE TABLE nft_listings (
    id SERIAL PRIMARY KEY,
    nft_id INTEGER REFERENCES nft_assets(id) ON DELETE CASCADE,
    seller_account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE,
    price DECIMAL(36, 18) NOT NULL,
    currency VARCHAR(32) NOT NULL,
    status VARCHAR(32) DEFAULT 'active',
    listed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP,
    sold_at TIMESTAMP,
    buyer_account_id INTEGER REFERENCES accounts(id)
);

CREATE INDEX idx_listings_nft ON nft_listings(nft_id);
CREATE INDEX idx_listings_status ON nft_listings(status);

CREATE TABLE nft_transactions (
    id SERIAL PRIMARY KEY,
    nft_id INTEGER REFERENCES nft_assets(id) ON DELETE CASCADE,
    from_account_id INTEGER REFERENCES accounts(id),
    to_account_id INTEGER REFERENCES accounts(id),
    tx_type VARCHAR(32) NOT NULL,
    tx_hash VARCHAR(128),
    chain VARCHAR(32) NOT NULL,
    amount DECIMAL(36, 18),
    currency VARCHAR(32),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_nft_tx_nft ON nft_transactions(nft_id);

CREATE TABLE bridge_requests (
    id SERIAL PRIMARY KEY,
    nft_id INTEGER REFERENCES nft_assets(id) ON DELETE CASCADE,
    from_chain VARCHAR(32) NOT NULL,
    to_chain VARCHAR(32) NOT NULL,
    from_tx_hash VARCHAR(128),
    to_tx_hash VARCHAR(128),
    status VARCHAR(32) DEFAULT 'pending',
    initiated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP
);

CREATE INDEX idx_bridge_nft ON bridge_requests(nft_id);
CREATE INDEX idx_bridge_status ON bridge_requests(status);

-- ============================================
-- ANTI-CHEAT AND MODERATION
-- ============================================

CREATE TABLE anticheat_logs (
    id SERIAL PRIMARY KEY,
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    detection_type VARCHAR(64) NOT NULL,
    severity VARCHAR(32) NOT NULL,
    details JSONB,
    action_taken VARCHAR(64),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_anticheat_character ON anticheat_logs(character_id);
CREATE INDEX idx_anticheat_type ON anticheat_logs(detection_type);

CREATE TABLE gm_actions (
    id SERIAL PRIMARY KEY,
    gm_account_id INTEGER REFERENCES accounts(id),
    target_account_id INTEGER REFERENCES accounts(id),
    target_character_id INTEGER REFERENCES characters(id),
    action_type VARCHAR(64) NOT NULL,
    reason TEXT,
    details JSONB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_gm_actions_gm ON gm_actions(gm_account_id);
CREATE INDEX idx_gm_actions_target ON gm_actions(target_account_id);

-- ============================================
-- MATCHMAKING
-- ============================================

CREATE TABLE matchmaking_ratings (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE PRIMARY KEY,
    rating INTEGER DEFAULT 1000,
    wins INTEGER DEFAULT 0,
    losses INTEGER DEFAULT 0,
    draws INTEGER DEFAULT 0,
    highest_rating INTEGER DEFAULT 1000,
    lowest_rating INTEGER DEFAULT 1000,
    streak INTEGER DEFAULT 0,
    last_match TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE match_history (
    id SERIAL PRIMARY KEY,
    realm_id INTEGER REFERENCES realms(id) ON DELETE CASCADE,
    match_type VARCHAR(32) NOT NULL,
    participants JSONB NOT NULL,
    winner_id INTEGER REFERENCES characters(id),
    duration INTEGER,
    details JSONB,
    played_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_matches_realm ON match_history(realm_id);
CREATE INDEX idx_matches_time ON match_history(played_at DESC);

-- ============================================
-- TRIGGERS AND FUNCTIONS
-- ============================================

-- Auto-update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply trigger to all tables with updated_at
CREATE TRIGGER update_accounts_updated_at BEFORE UPDATE ON accounts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_characters_updated_at BEFORE UPDATE ON characters
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_realms_updated_at BEFORE UPDATE ON realms
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_guilds_updated_at BEFORE UPDATE ON guilds
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_nft_assets_updated_at BEFORE UPDATE ON nft_assets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Update forum thread reply count
CREATE OR REPLACE FUNCTION update_thread_reply_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE forum_threads
        SET reply_count = reply_count + 1,
            last_reply_at = NEW.created_at,
            last_reply_by = NEW.character_name,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = NEW.thread_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE forum_threads
        SET reply_count = reply_count - 1,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = OLD.thread_id;
    END IF;
    RETURN NULL;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_thread_reply_count_trigger
AFTER INSERT OR DELETE ON forum_posts
    FOR EACH ROW EXECUTE FUNCTION update_thread_reply_count();

-- ============================================
-- DEFAULT DATA
-- ============================================

-- Insert default realm
INSERT INTO realms (name, slug, description, theme, pvp_type, region)
VALUES
    ('Shadowveil', 'shadowveil', 'Dark and mysterious realm', 'dark', 'open', 'us-east'),
    ('Aetheria', 'aetheria', 'Mythic and magical realm', 'mythic', 'optional', 'eu-west'),
    ('Warbound', 'warbound', 'Intense PvP warfare', 'war', 'hardcore', 'us-west'),
    ('Eternal Legacy', 'eternal-legacy', 'Classic nostalgic experience', 'classic', 'open', 'sa-east');

-- Insert default forum categories
INSERT INTO forum_categories (name, description, position)
VALUES
    ('Announcements', 'Official announcements and news', 1),
    ('General Discussion', 'General topics about the game', 2),
    ('Technical Support', 'Get help with technical issues', 3),
    ('Suggestions', 'Share your ideas and suggestions', 4),
    ('Bug Reports', 'Report bugs and issues', 5),
    ('Trading', 'In-game trading discussions', 6),
    ('Guilds', 'Guild recruitment and discussions', 7),
    ('Off-Topic', 'Non-game related discussions', 8);

-- Insert default quests
INSERT INTO quests (name, storage_key, storage_value, description)
VALUES
    ('Rookie Island Quest', 10001, 1, 'Complete the tutorial and leave Rookie Island'),
    ('First Blood', 10002, 1, 'Kill your first monster'),
    ('Dragon Slayer', 10003, 1, 'Defeat a mighty dragon');
