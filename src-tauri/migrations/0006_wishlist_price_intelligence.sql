-- Price intelligence columns for wishlist games
-- Separates Steam-specific data from all-shop data for accurate regional calculations

-- Steam historical low (from ITAD overview?shops=61)
-- This is what Steam itself has ever charged — discount % applies to our regional price
ALTER TABLE wishlist ADD COLUMN steam_historical_cut     INTEGER;
ALTER TABLE wishlist ADD COLUMN steam_historical_date    TEXT;

-- All-shop historical low (from ITAD historylow — informational only)
-- Cannot be used for regional price calculation — shop may not serve LK
-- or may not use Steam's regional pricing
ALTER TABLE wishlist ADD COLUMN all_time_low_cut         INTEGER;
ALTER TABLE wishlist ADD COLUMN all_time_low_shop        TEXT;
ALTER TABLE wishlist ADD COLUMN all_time_low_date        TEXT;

-- Our computed value: original_price × (1 - steam_historical_cut / 100)
-- Stored so we don't recompute on every read
-- This is the key differentiator — accurate regional historical low
ALTER TABLE wishlist ADD COLUMN predicted_regional_low   INTEGER;

-- Whether current price matches or beats predicted regional low
-- Updated every sync — drives the "At regional low ★" badge
ALTER TABLE wishlist ADD COLUMN is_at_regional_low       INTEGER NOT NULL DEFAULT 0;