-- This is the first database migration.
-- sqlx runs these in order, tracking which ones have been applied.
-- Never edit a migration after it's been run — add a new one instead.
 
-- ─────────────────────────────────────────────
-- GAMES
-- Core game data fetched from SteamSpy.
-- Updated daily by the background sync service.
-- ─────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS games (
    app_id              INTEGER PRIMARY KEY,    -- Steam App ID
    name                TEXT    NOT NULL,
    review_score        REAL,                   -- 0.0 - 100.0 (% positive)
    total_reviews       INTEGER,
    is_indie            INTEGER NOT NULL DEFAULT 0, -- SQLite has no BOOL: 0 = false, 1 = true
    price_current       INTEGER,                -- in cents, e.g. 999 = $9.99
    price_original      INTEGER,
    platform_windows    INTEGER NOT NULL DEFAULT 1,
    tags                TEXT,                  -- JSON array e.g. '["Indie","RPG"]'
    last_updated        TIMESTAMP
);

-- Index for the most common query: fetch top-rated indie games
CREATE INDEX IF NOT EXISTS idx_games_score ON games(review_score DESC);
CREATE INDEX IF NOT EXISTS idx_games_indie ON games (is_indie);

-- ─────────────────────────────────────────────
-- WISHLIST
-- Games the user has wishlisted on Steam.
-- Fetched from their public Steam profile.
-- ─────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS wishlist (
    app_id              INTEGER PRIMARY KEY,
    name                TEXT    NOT NULL,
    review_summary      TEXT,                 -- e.g. "Overwhelmingly Positive"
    reviews_percent     INTEGER,
    reviews_total       INTEGER,
    date_added          INTEGER,              -- Unix timestamp
    current_price       INTEGER,             -- in cents
    historical_low      INTEGER,             -- in cents
    discount_percent    INTEGER,
    last_checked        TIMESTAMP
);

-- ─────────────────────────────────────────────
-- PRICE HISTORY
-- Append-only price records over time.
-- Never UPDATE or DELETE — only INSERT.
-- Used to detect sale patterns and predict future sales.
-- ─────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS price_history (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    app_id              INTEGER NOT NULL,
    price               INTEGER NOT NULL,     -- in cents
    discount_percent    INTEGER NOT NULL DEFAULT 0,
    recorded_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    source              TEXT NOT NULL         -- 'steam' | 'isthereanydeal'
);

CREATE INDEX IF NOT EXISTS idx_price_history_app ON price_history (app_id);
CREATE INDEX IF NOT EXISTS idx_price_history_date ON price_history (recorded_at DESC);

-- ─────────────────────────────────────────────
-- USER SETTINGS
-- Simple key-value store for user preferences.
-- One row per setting, identified by key name.
-- ─────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS user_settings (
    key     TEXT PRIMARY KEY,
    value   TEXT NOT NULL
);

-- Insert defaults so the app works before the user configures anything
INSERT OR IGNORE INTO user_settings (key, value) VALUES
    ('min_review_score',      '90.0'),
    ('min_discount_percent',  '50'),
    ('sync_interval_hours',   '24'),
    ('alert_threshold_percent', '50');