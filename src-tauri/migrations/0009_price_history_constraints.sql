DELETE FROM price_history
WHERE rowid NOT IN (
    SELECT MIN(rowid)
    FROM price_history
    GROUP BY app_id, recorded_at, source
);
-- This prevents duplicate inserts even if our logic has a bug
-- INSERT OR IGNORE will silently skip duplicates
CREATE UNIQUE INDEX IF NOT EXISTS idx_price_history_unique
ON price_history (app_id, recorded_at, source);

ALTER TABLE wishlist ADD COLUMN itad_history_bootstrapped INTEGER NOT NULL DEFAULT 0;