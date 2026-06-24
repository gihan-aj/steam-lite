-- Review data (computed label from score + count)
ALTER TABLE wishlist ADD COLUMN steam_review_score   INTEGER;  -- 0-100
ALTER TABLE wishlist ADD COLUMN steam_review_count   INTEGER;
ALTER TABLE wishlist ADD COLUMN review_label         TEXT;     -- computed e.g. "Very Positive"
ALTER TABLE wishlist ADD COLUMN opencritic_score     INTEGER;
ALTER TABLE wishlist ADD COLUMN metacritic_score     INTEGER;

-- Game metadata
ALTER TABLE wishlist ADD COLUMN release_date         TEXT;     -- "2021-12-16"
ALTER TABLE wishlist ADD COLUMN tags                 TEXT;     -- JSON array ["RPG","Action"]
ALTER TABLE wishlist ADD COLUMN developers           TEXT;     -- JSON array
ALTER TABLE wishlist ADD COLUMN itad_id              TEXT;     -- store UUID to avoid re-lookup

-- Sale pattern stats (computed from price history)
ALTER TABLE wishlist ADD COLUMN avg_sale_interval_days  INTEGER;
ALTER TABLE wishlist ADD COLUMN typical_discount_min    INTEGER;
ALTER TABLE wishlist ADD COLUMN typical_discount_max    INTEGER;
ALTER TABLE wishlist ADD COLUMN last_sale_date          TEXT;
ALTER TABLE wishlist ADD COLUMN predicted_next_sale     TEXT;   -- "2026-09"