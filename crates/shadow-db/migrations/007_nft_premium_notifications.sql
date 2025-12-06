-- Migration: NFT, Premium, Notifications, 2FA, and Wallet Auth tables
-- Version: 007

-- ============================================
-- NFT Tables
-- ============================================

-- Blockchain chain enum
DO $$ BEGIN
    CREATE TYPE blockchain_chain AS ENUM ('ethereum', 'polygon', 'starknet', 'bitcoin', 'base', 'arbitrum');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- NFT status enum
DO $$ BEGIN
    CREATE TYPE nft_status AS ENUM ('minted', 'listed', 'transferred', 'burned');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- NFTs table
CREATE TABLE IF NOT EXISTS nfts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_id VARCHAR(100) NOT NULL UNIQUE,
    chain blockchain_chain NOT NULL DEFAULT 'polygon',
    contract_address VARCHAR(100) NOT NULL,
    owner_address VARCHAR(100) NOT NULL,
    item_id INTEGER REFERENCES items(id),
    nft_type VARCHAR(50) NOT NULL DEFAULT 'item',
    rarity VARCHAR(20) NOT NULL DEFAULT 'common',
    metadata_name VARCHAR(200) NOT NULL,
    metadata_description TEXT NOT NULL,
    metadata_image VARCHAR(500) NOT NULL,
    metadata_animation_url VARCHAR(500),
    metadata_external_url VARCHAR(500),
    listing_price VARCHAR(50),
    listing_currency VARCHAR(20),
    status nft_status NOT NULL DEFAULT 'minted',
    minted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_transfer_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_nfts_owner ON nfts(owner_address);
CREATE INDEX IF NOT EXISTS idx_nfts_chain ON nfts(chain);
CREATE INDEX IF NOT EXISTS idx_nfts_status ON nfts(status);
CREATE INDEX IF NOT EXISTS idx_nfts_item ON nfts(item_id);

-- NFT transfers log
CREATE TABLE IF NOT EXISTS nft_transfers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nft_id UUID NOT NULL REFERENCES nfts(id),
    from_address VARCHAR(100) NOT NULL,
    to_address VARCHAR(100) NOT NULL,
    tx_hash VARCHAR(100),
    price VARCHAR(50),
    currency VARCHAR(20),
    transferred_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_nft_transfers_nft ON nft_transfers(nft_id);

-- ============================================
-- Premium Tables
-- ============================================

-- Add premium columns to accounts if not exists
DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'accounts' AND column_name = 'premium_until') THEN
        ALTER TABLE accounts ADD COLUMN premium_until TIMESTAMP WITH TIME ZONE;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'accounts' AND column_name = 'premium_plan') THEN
        ALTER TABLE accounts ADD COLUMN premium_plan VARCHAR(20);
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'accounts' AND column_name = 'coins') THEN
        ALTER TABLE accounts ADD COLUMN coins BIGINT DEFAULT 0;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'accounts' AND column_name = 'auto_renew') THEN
        ALTER TABLE accounts ADD COLUMN auto_renew BOOLEAN DEFAULT false;
    END IF;
END $$;

-- Premium transactions
CREATE TABLE IF NOT EXISTS premium_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    transaction_type VARCHAR(50) NOT NULL,
    description TEXT NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(10) NOT NULL DEFAULT 'USD',
    status VARCHAR(20) NOT NULL DEFAULT 'completed',
    payment_provider VARCHAR(50),
    payment_reference VARCHAR(200),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_premium_tx_account ON premium_transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_premium_tx_date ON premium_transactions(created_at);

-- ============================================
-- Notification Tables
-- ============================================

-- Notification type enum
DO $$ BEGIN
    CREATE TYPE notification_type AS ENUM ('levelup', 'trade', 'achievement', 'guild', 'system', 'message', 'friend', 'death', 'market');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Notifications table
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    notification_type notification_type NOT NULL DEFAULT 'system',
    title VARCHAR(200) NOT NULL,
    message TEXT NOT NULL,
    action_url VARCHAR(500),
    data JSONB,
    read_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_notifications_account ON notifications(account_id);
CREATE INDEX IF NOT EXISTS idx_notifications_unread ON notifications(account_id) WHERE read_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_notifications_type ON notifications(notification_type);
CREATE INDEX IF NOT EXISTS idx_notifications_date ON notifications(created_at);

-- ============================================
-- 2FA Tables
-- ============================================

-- Add 2FA columns to accounts
DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'accounts' AND column_name = 'totp_secret') THEN
        ALTER TABLE accounts ADD COLUMN totp_secret VARCHAR(100);
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'accounts' AND column_name = 'totp_pending_secret') THEN
        ALTER TABLE accounts ADD COLUMN totp_pending_secret VARCHAR(100);
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'accounts' AND column_name = 'totp_enabled') THEN
        ALTER TABLE accounts ADD COLUMN totp_enabled BOOLEAN DEFAULT false;
    END IF;
END $$;

-- ============================================
-- Wallet Authentication Tables
-- ============================================

-- Wallet nonces for authentication
CREATE TABLE IF NOT EXISTS wallet_nonces (
    address VARCHAR(100) PRIMARY KEY,
    nonce VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Account wallets (linking wallets to accounts)
CREATE TABLE IF NOT EXISTS account_wallets (
    id SERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    wallet_address VARCHAR(100) NOT NULL UNIQUE,
    chain VARCHAR(50) NOT NULL DEFAULT 'ethereum',
    primary_wallet BOOLEAN DEFAULT false,
    verified BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_account_wallet UNIQUE (account_id, wallet_address)
);

CREATE INDEX IF NOT EXISTS idx_account_wallets_account ON account_wallets(account_id);
CREATE INDEX IF NOT EXISTS idx_account_wallets_address ON account_wallets(wallet_address);

-- ============================================
-- Sample Data
-- ============================================

-- Insert some sample notifications (if accounts exist)
INSERT INTO notifications (account_id, notification_type, title, message, data)
SELECT 1, 'system'::notification_type, 'Welcome to Shadow OT!', 'Your adventure begins now. Explore the realms and become a legend!', '{"type": "welcome"}'::jsonb
WHERE EXISTS (SELECT 1 FROM accounts WHERE id = 1)
ON CONFLICT DO NOTHING;

-- Create NFT metadata attributes table for complex metadata
CREATE TABLE IF NOT EXISTS nft_attributes (
    id SERIAL PRIMARY KEY,
    nft_id UUID NOT NULL REFERENCES nfts(id) ON DELETE CASCADE,
    trait_type VARCHAR(100) NOT NULL,
    value TEXT NOT NULL,
    display_type VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_nft_attributes_nft ON nft_attributes(nft_id);

-- ============================================
-- Update timestamps trigger
-- ============================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS update_nfts_updated_at ON nfts;
CREATE TRIGGER update_nfts_updated_at
    BEFORE UPDATE ON nfts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
