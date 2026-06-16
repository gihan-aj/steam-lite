use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Raw database row for the `games` table.
/// Used exclusively by GameRepository — not sent to the frontend.
#[derive(Debug, Clone)]
pub struct GameRow {
    pub app_id:           i64,
    pub name:             String,
    pub review_score:     Option<f64>,   // REAL → f64
    pub total_reviews:    Option<i64>,   // INTEGER → i64
    pub is_indie:         i64,           // BOOLEAN stored as 0/1
    pub price_current:    Option<i64>,   // INTEGER (cents)
    pub price_original:   Option<i64>,
    pub platform_windows: i64,           // BOOLEAN stored as 0/1
    pub tags:             Option<String>,
    pub last_updated:     Option<DateTime<Utc>>,
}

/// Raw database row for the `wishlist` table.
#[derive(Debug, Clone)]
pub struct WishlistRow {
    pub app_id:           i64,
    pub name:             String,
    pub review_summary:   Option<String>,
    pub reviews_percent:  Option<i64>,
    pub reviews_total:    Option<i64>,
    pub date_added:       Option<i64>,
    pub current_price:    Option<i64>,
    pub original_price:   Option<i64>,
    pub historical_low:   Option<i64>,
    pub discount_percent: Option<i64>,
}

/// Raw database row for the `price_history` table.
#[derive(Debug, Clone)]
pub struct PriceRow {
    pub app_id:           i64,
    pub price:            i64,
    pub discount_percent: i64,
    pub recorded_at:      DateTime<Utc>,
    pub source:           String,
}

/// Represents a Steam game as stored in our local database.
/// This is what we persist after fetching from SteamSpy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Steam App ID — unique identifier for every game on Steam
    pub app_id:             i64,
 
    pub name:               String,
 
    /// Positive review percentage (0.0 - 100.0).
    /// Option because SteamSpy may not have data for very new games.
    pub review_score:       Option<f64>,
 
    pub total_reviews:      Option<i64>,
 
    /// Whether this game is tagged as indie by SteamSpy
    pub is_indie:           bool,
 
    /// Current price in USD (cents, to avoid floating point issues)
    /// e.g. 999 = $9.99
    pub price_current:      Option<i64>,
 
    pub price_original:     Option<i64>,
 
    /// Whether the game runs on Windows
    pub platform_windows:   bool,
 
    /// JSON-encoded list of genre/category tags
    /// e.g. ["Indie", "RPG", "Singleplayer"]
    pub tags:               Option<String>,
 
    /// When we last fetched data for this game from the API
    pub last_updated:       Option<DateTime<Utc>>,
}

impl From<GameRow> for Game {
    fn from(row: GameRow) -> Self {
        Game {
            app_id:           row.app_id,
            name:             row.name,
            review_score:     row.review_score,
            total_reviews:    row.total_reviews,
            is_indie:         row.is_indie != 0,         // 0 = false, anything else = true
            price_current:    row.price_current,
            price_original:   row.price_original,
            platform_windows: row.platform_windows != 0,
            tags:             row.tags,
            last_updated:     row.last_updated,
        }
    }
}

// A game enriched with a recommendation score, ready to show in the UI.
/// Note: this is a SEPARATE struct from Game — it's only used for display,
/// not stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedGame {
    pub app_id: i64,
    pub name: String,
    pub review_score: f64,
    pub total_reviews: i64,
    pub price_current: Option<i64>,
    pub price_original: Option<i64>,
    pub tags: Vec<String>,
 
    /// Our calculated score — higher is better.
    /// Combines review score (60%), review count (30%), recency (10%).
    pub recommendation_score: f64,
 
    /// Human-readable discount string e.g. "-75%"
    pub discount_label: Option<String>,
}
 
/// A single item from a user's Steam wishlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WishlistItem {
    pub app_id:             i64,
    pub name:               String,
 
    /// Steam's review summary e.g. "Overwhelmingly Positive"
    pub review_summary:     Option<String>,
 
    pub reviews_percent:    Option<i64>,
    pub reviews_total:      Option<i64>,
 
    /// Unix timestamp of when the game was added to the wishlist
    pub date_added:         Option<i64>,
 
    /// Current price from our price tracking
    pub current_price:      Option<i64>,

    /// The original price of the game
    pub original_price:      Option<i64>,
 
    /// The all-time historical low price
    pub historical_low:     Option<i64>,
 
    /// Current discount percentage (0-100)
    pub discount_percent:   Option<i64>,
 
    /// Our prediction on whether to buy now or wait
    pub buy_recommendation: Option<BuyRecommendation>,
}

impl From<WishlistRow> for WishlistItem {
    fn from(row: WishlistRow) -> Self {
        WishlistItem {
            app_id:             row.app_id,
            name:               row.name,
            review_summary:     row.review_summary,
            reviews_percent:    row.reviews_percent,
            reviews_total:      row.reviews_total,
            date_added:         row.date_added,
            current_price:      row.current_price,
            original_price:     row.original_price,
            historical_low:     row.historical_low,
            discount_percent:   row.discount_percent,
            buy_recommendation: None, // filled in by the service layer later
        }
    }
}
 
/// The result of our price prediction algorithm for a single game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyRecommendation {
    /// "Buy now" | "Wait" | "Best price" | "Watch"
    pub action:             String,
 
    /// 0-100: how strongly we recommend waiting (100 = definitely wait)
    pub wait_score:         u8,
 
    /// Plain-language explanation shown to the user
    pub reason:             String,
 
    /// Our estimate of when the next sale might occur
    pub predicted_next_sale: Option<String>,
}
 
/// A single price data point stored in price_history table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub app_id:             i64,
 
    /// Price in cents (e.g. 999 = $9.99)
    pub price:              i64,
 
    pub discount_percent:   i64,
    pub recorded_at:        DateTime<Utc>,
 
    /// Which source this came from: "steam" | "isthereanydeal"
    pub source:             String,
}

impl From<PriceRow> for PricePoint {
    fn from(row: PriceRow) -> Self {
        PricePoint {
            app_id:           row.app_id,
            price:            row.price,
            discount_percent: row.discount_percent,
            recorded_at:      row.recorded_at,
            source:           row.source,
        }
    }
}
 
/// User-configurable settings, stored in the user_settings table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    /// The user's Steam ID (64-bit numeric string)
    /// e.g. "76561198012345678"
    pub steam_id:               Option<String>,

    /// The user's Steam API key (required for some features)
    pub steam_api_key:          Option<String>,
 
    /// Minimum review score % to show in recommendations (default: 90)
    pub min_review_score:       f64,
 
    /// Minimum discount % to show in discount tracker (default: 50)
    pub min_discount_percent:   i64,
 
    /// How often to sync game data in hours (default: 24)
    pub sync_interval_hours:    i64,
 
    /// Price drop alert threshold % (default: 50)
    pub alert_threshold_percent: i64,
}
 
impl UserSettings {
    /// Returns the default settings for a new user.
    pub fn new() -> Self {
        Self::default()
    }
}
 
impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            steam_id: None,  
            steam_api_key: None,
            min_review_score: 90.0,
            min_discount_percent: 50,
            sync_interval_hours: 24,
            alert_threshold_percent: 50,
        }
    }
}
 
// ─────────────────────────────────────────────
// UTILITY FUNCTIONS
// ─────────────────────────────────────────────
/// Helper to convert a price in cents to a display string.
pub fn format_price(cents: i64) -> String {
    if cents == 0 {
        "Free".to_string()
    } else {
        format!("${:.2}", cents as f64 / 100.0)
    }
}
 
/// Calculate discount percentage given original and current price.
pub fn calculate_discount(original: i64, current: i64) -> i64 {
    if original == 0 {
        return 0;
    }
    (((original - current) as f64 / original as f64) * 100.0) as i64
}