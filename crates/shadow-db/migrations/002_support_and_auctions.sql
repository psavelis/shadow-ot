-- Shadow OT Support Tickets and Auctions Migration
-- Adds support tickets, auction tables, and related types

-- ============================================
-- ENUMS
-- ============================================

-- Support ticket enums
CREATE TYPE ticket_category AS ENUM ('technical', 'billing', 'account', 'report', 'other');
CREATE TYPE ticket_status AS ENUM ('open', 'pending', 'resolved', 'closed');
CREATE TYPE ticket_priority AS ENUM ('low', 'medium', 'high', 'urgent');

-- Auction enums
CREATE TYPE auction_type AS ENUM ('character', 'item');
CREATE TYPE auction_status AS ENUM ('active', 'ended', 'cancelled', 'won');

-- Vocation enum (if not exists)
DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'vocation') THEN
        CREATE TYPE vocation AS ENUM ('none', 'knight', 'paladin', 'sorcerer', 'druid', 'eliteknight', 'royalpaladin', 'mastersorcerer', 'elderdruid');
    END IF;
END $$;

-- ============================================
-- SUPPORT TICKETS
-- ============================================

CREATE TABLE support_tickets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE NOT NULL,
    subject VARCHAR(255) NOT NULL,
    category ticket_category NOT NULL DEFAULT 'other',
    status ticket_status NOT NULL DEFAULT 'open',
    priority ticket_priority NOT NULL DEFAULT 'medium',
    assigned_to INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_support_tickets_account ON support_tickets(account_id);
CREATE INDEX idx_support_tickets_status ON support_tickets(status);
CREATE INDEX idx_support_tickets_priority ON support_tickets(priority);
CREATE INDEX idx_support_tickets_created ON support_tickets(created_at DESC);
CREATE INDEX idx_support_tickets_updated ON support_tickets(updated_at DESC);

CREATE TABLE ticket_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ticket_id UUID REFERENCES support_tickets(id) ON DELETE CASCADE NOT NULL,
    author_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
    author_type VARCHAR(32) NOT NULL DEFAULT 'user', -- 'user', 'support', 'system'
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    edited_at TIMESTAMP WITH TIME ZONE,
    is_internal BOOLEAN DEFAULT FALSE -- Internal staff notes
);

CREATE INDEX idx_ticket_messages_ticket ON ticket_messages(ticket_id);
CREATE INDEX idx_ticket_messages_created ON ticket_messages(created_at ASC);

CREATE TABLE ticket_attachments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    message_id UUID REFERENCES ticket_messages(id) ON DELETE CASCADE NOT NULL,
    filename VARCHAR(255) NOT NULL,
    file_url TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type VARCHAR(128) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ticket_attachments_message ON ticket_attachments(message_id);

-- ============================================
-- CHARACTER AUCTIONS
-- ============================================

CREATE TABLE character_auctions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    character_id INTEGER REFERENCES characters(id) ON DELETE SET NULL,
    seller_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL NOT NULL,
    
    -- Character snapshot at time of auction
    character_name VARCHAR(64) NOT NULL,
    level INTEGER NOT NULL,
    vocation vocation NOT NULL,
    skill_fist INTEGER NOT NULL DEFAULT 10,
    skill_club INTEGER NOT NULL DEFAULT 10,
    skill_sword INTEGER NOT NULL DEFAULT 10,
    skill_axe INTEGER NOT NULL DEFAULT 10,
    skill_distance INTEGER NOT NULL DEFAULT 10,
    skill_shielding INTEGER NOT NULL DEFAULT 10,
    skill_fishing INTEGER NOT NULL DEFAULT 10,
    skill_magic INTEGER NOT NULL DEFAULT 0,
    
    -- Auction details
    min_bid BIGINT NOT NULL,
    current_bid BIGINT NOT NULL DEFAULT 0,
    bid_increment BIGINT NOT NULL DEFAULT 100,
    bid_count INTEGER NOT NULL DEFAULT 0,
    winner_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
    
    -- Timing
    starts_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ends_at TIMESTAMP WITH TIME ZONE NOT NULL,
    status auction_status NOT NULL DEFAULT 'active',
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_char_auctions_seller ON character_auctions(seller_id);
CREATE INDEX idx_char_auctions_status ON character_auctions(status);
CREATE INDEX idx_char_auctions_ends ON character_auctions(ends_at);
CREATE INDEX idx_char_auctions_level ON character_auctions(level);
CREATE INDEX idx_char_auctions_vocation ON character_auctions(vocation);

-- ============================================
-- ITEM AUCTIONS
-- ============================================

CREATE TABLE item_auctions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    seller_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL NOT NULL,
    seller_name VARCHAR(64) NOT NULL,
    
    -- Item details
    item_id INTEGER NOT NULL,
    item_name VARCHAR(255) NOT NULL,
    item_count INTEGER NOT NULL DEFAULT 1,
    is_nft BOOLEAN NOT NULL DEFAULT FALSE,
    nft_token_id VARCHAR(255),
    nft_contract_address VARCHAR(255),
    
    -- Auction details  
    min_bid BIGINT NOT NULL,
    current_bid BIGINT NOT NULL DEFAULT 0,
    bid_increment BIGINT NOT NULL DEFAULT 100,
    bid_count INTEGER NOT NULL DEFAULT 0,
    winner_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
    
    -- Timing
    starts_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ends_at TIMESTAMP WITH TIME ZONE NOT NULL,
    status auction_status NOT NULL DEFAULT 'active',
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_item_auctions_seller ON item_auctions(seller_id);
CREATE INDEX idx_item_auctions_status ON item_auctions(status);
CREATE INDEX idx_item_auctions_ends ON item_auctions(ends_at);
CREATE INDEX idx_item_auctions_item ON item_auctions(item_id);
CREATE INDEX idx_item_auctions_nft ON item_auctions(is_nft) WHERE is_nft = TRUE;

-- ============================================
-- AUCTION BIDS
-- ============================================

CREATE TABLE auction_bids (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    auction_id UUID NOT NULL,
    auction_type auction_type NOT NULL,
    bidder_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL NOT NULL,
    amount BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Self-referential for outbid notifications
    outbid_by UUID REFERENCES auction_bids(id)
);

CREATE INDEX idx_auction_bids_auction ON auction_bids(auction_id, auction_type);
CREATE INDEX idx_auction_bids_bidder ON auction_bids(bidder_id);
CREATE INDEX idx_auction_bids_amount ON auction_bids(amount DESC);

-- ============================================
-- USER ITEMS (for item auctions)
-- ============================================

CREATE TABLE IF NOT EXISTS user_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE NOT NULL,
    item_id INTEGER NOT NULL,
    count INTEGER NOT NULL DEFAULT 1,
    nft_token_id VARCHAR(255),
    acquired_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(account_id, item_id, nft_token_id)
);

CREATE INDEX idx_user_items_account ON user_items(account_id);
CREATE INDEX idx_user_items_item ON user_items(item_id);

-- ============================================
-- FUNCTIONS AND TRIGGERS
-- ============================================

-- Update timestamp trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Auto-update updated_at for support tickets
CREATE TRIGGER update_support_tickets_updated_at
    BEFORE UPDATE ON support_tickets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Auto-update updated_at for character auctions
CREATE TRIGGER update_character_auctions_updated_at
    BEFORE UPDATE ON character_auctions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Auto-update updated_at for item auctions
CREATE TRIGGER update_item_auctions_updated_at
    BEFORE UPDATE ON item_auctions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================
-- FAQ TABLE (optional, for database-driven FAQ)
-- ============================================

CREATE TABLE faq_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE faq_items (
    id SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES faq_categories(id) ON DELETE CASCADE NOT NULL,
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0,
    is_published BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_faq_items_category ON faq_items(category_id);
CREATE INDEX idx_faq_items_published ON faq_items(is_published);

-- Insert default FAQ categories
INSERT INTO faq_categories (name, display_order) VALUES
    ('Getting Started', 1),
    ('Account & Security', 2),
    ('Gameplay', 3),
    ('Premium & Shop', 4);

-- ============================================
-- AUCTION NOTIFICATIONS
-- ============================================

CREATE TABLE auction_notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE NOT NULL,
    auction_id UUID NOT NULL,
    auction_type auction_type NOT NULL,
    notification_type VARCHAR(32) NOT NULL, -- 'outbid', 'won', 'ended', 'cancelled'
    message TEXT NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_auction_notifications_account ON auction_notifications(account_id);
CREATE INDEX idx_auction_notifications_unread ON auction_notifications(account_id, is_read) WHERE is_read = FALSE;

-- ============================================
-- AUCTION WATCHLIST
-- ============================================

CREATE TABLE auction_watchlist (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE NOT NULL,
    auction_id UUID NOT NULL,
    auction_type auction_type NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(account_id, auction_id, auction_type)
);

CREATE INDEX idx_auction_watchlist_account ON auction_watchlist(account_id);
