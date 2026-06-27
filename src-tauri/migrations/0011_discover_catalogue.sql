CREATE TABLE IF NOT EXISTS crawl_state (
    key     TEXT PRIMARY KEY,
    value   TEXT NOT NULL
);

INSERT OR IGNORE INTO crawl_state (key, value) VALUES
    ('current_page',    '0'),
    ('total_pages',     '50'),
    ('status',          'idle'),    -- idle | running | paused | complete
    ('last_page_at',    ''),        -- ISO timestamp of last successful page
    ('game_indexed',    '0'),       -- total games stored so far
    ('game_qualified',  '0');       -- games that passed our score filter

-- Add gem_score and crawl metadata to games table
ALTER TABLE games ADD COLUMN gem_score       REAL;
ALTER TABLE games ADD COLUMN owners_lower    INTEGER;  -- parsed from "50,000 .. 100,000"
ALTER TABLE games ADD COLUMN avg_playtime    INTEGER;  -- average_forever from SteamSpy (minutes)
ALTER TABLE games ADD COLUMN crawl_source    TEXT;     -- 'top100' | 'all_page_N'
ALTER TABLE games ADD COLUMN header_image    TEXT;     -- from Steam appdetails
ALTER TABLE games ADD COLUMN short_desc      TEXT;     -- from Steam appdetails
ALTER TABLE games ADD COLUMN genres          TEXT;     -- JSON array from Steam appdetails