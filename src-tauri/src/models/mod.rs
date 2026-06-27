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
    pub gem_score:        Option<f64>,
    pub owners_lower:     Option<i64>,
    pub avg_playtime:     Option<i64>,
    pub crawl_source:     Option<String>,
    pub header_image:     Option<String>,
    pub short_desc:       Option<String>,
    pub genres:           Option<String>,
}

/// Raw database row for the `wishlist` table.
#[derive(Debug, Clone)]
pub struct WishlistRow {
    pub app_id:                 i64,
    pub name:                   String,
    pub review_summary:         Option<String>,
    pub reviews_percent:        Option<i64>,
    pub reviews_total:          Option<i64>,
    pub date_added:             Option<i64>,
    pub current_price:          Option<i64>,
    pub original_price:         Option<i64>,
    pub historical_low:         Option<i64>,
    pub discount_percent:       Option<i64>,
    pub header_image:           Option<String>,
    pub short_description:      Option<String>,
    pub steam_historical_cut:   Option<i64>,
    pub steam_historical_date:  Option<String>,
    pub all_time_low_cut:       Option<i64>,
    pub all_time_low_shop:      Option<String>,
    pub all_time_low_date:      Option<String>,
    pub predicted_regional_low: Option<i64>,
    pub is_at_regional_low:     i64, 
    pub steam_review_score:      Option<i64>,
    pub steam_review_count:      Option<i64>,
    pub review_label:            Option<String>,
    pub opencritic_score:        Option<i64>,
    pub metacritic_score:        Option<i64>,
    pub release_date:            Option<String>,
    pub tags:                    Option<String>,   // JSON
    pub developers:              Option<String>,   // JSON
    pub itad_id:                 Option<String>,
    pub avg_sale_interval_days:  Option<i64>,
    pub typical_discount_min:    Option<i64>,
    pub typical_discount_max:    Option<i64>,
    pub last_sale_date:          Option<String>,
    pub predicted_next_sale:     Option<String>,
    pub itad_history_bootstrapped: i64,
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
    pub review_score:       Option<f64>,
    pub total_reviews:      Option<i64>,
    pub is_indie:           bool,
    pub price_current:      Option<i64>,
    pub price_original:     Option<i64>,
    pub platform_windows:   bool,
    pub tags:               Option<String>,
    pub last_updated:       Option<DateTime<Utc>>,
    pub gem_score:          Option<f64>,
    pub owners_lower:       Option<i64>,
    pub avg_playtime:       Option<i64>,
    pub crawl_source:       Option<String>,
    pub header_image:       Option<String>,
    pub short_desc:         Option<String>,
    pub genres:             Option<String>,
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
            gem_score:        row.gem_score,
            owners_lower:     row.owners_lower,
            avg_playtime:     row.avg_playtime,
            crawl_source:     row.crawl_source,
            header_image:     row.header_image,
            short_desc:       row.short_desc,
            genres:           row.genres
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
    pub app_id:                 i64,
    pub name:                   String,
    pub review_summary:         Option<String>,
    pub reviews_percent:        Option<i64>,
    pub reviews_total:          Option<i64>,
    pub date_added:             Option<i64>,
    pub current_price:          Option<i64>,
    pub original_price:         Option<i64>,
    pub historical_low:         Option<i64>,
    pub discount_percent:       Option<i64>,
    pub header_image:           Option<String>,
    pub short_description:      Option<String>,
    pub buy_recommendation:     Option<BuyRecommendation>,
    pub steam_historical_cut:   Option<i64>,
    pub steam_historical_date:  Option<String>,
    pub all_time_low_cut:       Option<i64>,
    pub all_time_low_shop:      Option<String>,
    pub all_time_low_date:      Option<String>,
    pub predicted_regional_low: Option<i64>,
    pub is_at_regional_low:     bool,
    pub price_signal:           Option<PriceSignal>,
    pub itad_discrepancy:       Option<i64>,
    pub steam_review_score:      Option<i64>,
    pub steam_review_count:      Option<i64>,
    pub review_label:            Option<String>,
    pub opencritic_score:        Option<i64>,
    pub metacritic_score:        Option<i64>,
    pub release_date:            Option<String>,
    pub tags:                    Vec<String>,
    pub developers:              Vec<String>,
    pub itad_id:                 Option<String>,
    pub avg_sale_interval_days:  Option<i64>,
    pub typical_discount_min:    Option<i64>,
    pub typical_discount_max:    Option<i64>,
    pub last_sale_date:          Option<String>,
    pub predicted_next_sale:     Option<String>,
    pub itad_history_bootstrapped: bool,
}

impl From<WishlistRow> for WishlistItem {
    fn from(row: WishlistRow) -> Self {

        let tags: Vec<String> = row.tags
            .as_deref()
            .and_then(|t| serde_json::from_str(t).ok())
            .unwrap_or_default();

        let developers: Vec<String> = row.developers
            .as_deref()
            .and_then(|d| serde_json::from_str(d).ok())
            .unwrap_or_default();

        WishlistItem {
            app_id:                 row.app_id,
            name:                   row.name,
            review_summary:         row.review_summary,
            reviews_percent:        row.reviews_percent,
            reviews_total:          row.reviews_total,
            date_added:             row.date_added,
            current_price:          row.current_price,
            original_price:         row.original_price,
            historical_low:         row.historical_low,
            discount_percent:       row.discount_percent,
            header_image:           row.header_image,
            short_description:      row.short_description,
            buy_recommendation:     None,
            steam_historical_cut:   row.steam_historical_cut,
            steam_historical_date:  row.steam_historical_date,
            all_time_low_cut:       row.all_time_low_cut,
            all_time_low_shop:      row.all_time_low_shop,
            all_time_low_date:      row.all_time_low_date,
            predicted_regional_low: row.predicted_regional_low,
            is_at_regional_low:     row.is_at_regional_low != 0,
            price_signal:           None, 
            itad_discrepancy:       None,
            steam_review_score:     row.steam_review_score,
            steam_review_count:     row.steam_review_count,
            review_label:           row.review_label,
            opencritic_score:       row.opencritic_score,
            metacritic_score:       row.metacritic_score,
            release_date:           row.release_date,
            tags,
            developers,
            itad_id:                row.itad_id,
            avg_sale_interval_days: row.avg_sale_interval_days,
            typical_discount_min:   row.typical_discount_min,
            typical_discount_max:   row.typical_discount_max,
            last_sale_date:         row.last_sale_date,
            predicted_next_sale:    row.predicted_next_sale,
            itad_history_bootstrapped: row.itad_history_bootstrapped != 0,
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

/// Generate a human-readable "buy signal" for the UI.
///
/// Returns a structured recommendation the wishlist card can display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceSignal {
    /// Primary badge text e.g. "★ Regional Low", "↓ Below historical low"
    pub badge: String,
    /// Badge colour category: "green", "yellow", "blue", "none"
    pub level: String,
    /// Supporting detail e.g. "Best Steam price: -65% (Jun 2026)"
    pub detail: Option<String>,
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

    /// Is There Any Deal API key (optional for some features)
    pub itad_api_key:           Option<String>,

    /// The user's country code
    pub country_code:           String,
 
    /// Minimum review score % to show in recommendations (default: 90)
    pub min_review_score:       f64,
 
    /// Minimum discount % to show in discount tracker (default: 50)
    pub min_discount_percent:   i64,
 
    /// How often to sync game data in hours (default: 24)
    pub sync_interval_hours:    i64,

    // When was last sync
    pub last_synced_at:         Option<DateTime<Utc>>,
 
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
            itad_api_key: None,
            country_code: "lk".to_string(),
            min_review_score: 90.0,
            min_discount_percent: 50,
            sync_interval_hours: 24,
            last_synced_at: None,
            alert_threshold_percent: 50,
        }
    }
}
 
// ─────────────────────────────────────────────
// UTILITY FUNCTIONS
// ─────────────────────────────────────────────
// Helper to convert a price in cents to a display string.
// pub fn format_price(cents: i64) -> String {
//     if cents == 0 {
//         "Free".to_string()
//     } else {
//         format!("${:.2}", cents as f64 / 100.0)
//     }
// }
 
// Calculate discount percentage given original and current price.
// pub fn calculate_discount(original: i64, current: i64) -> i64 {
//     if original == 0 {
//         return 0;
//     }
//     (((original - current) as f64 / original as f64) * 100.0) as i64
// }

/// Tracks the progress of the background catalogue crawl.
/// Stored in the crawl_state table as key-value pairs.
/// Lets us resume after interruption, crash, or app close.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrawlState {
    /// Which SteamSpy /all page to fetch next (0-indexed)
    pub current_page:    u32,

    /// Estimated total pages (SteamSpy has ~50)
    pub total_pages:     u32,

    /// Current status of the crawl
    pub status:          CrawlStatus,

    /// When the last page was successfully fetched
    pub last_page_at:    Option<String>,

    /// Total games stored in the games table so far
    pub games_indexed:   u32,

    /// Games that passed our hidden gem score threshold
    pub games_qualified: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CrawlStatus {
    #[default]
    Idle,       // never started
    Running,    // actively fetching pages
    Paused,     // interrupted (app closed mid-crawl)
    Complete,   // all pages fetched
}

// Display for logging
impl std::fmt::Display for CrawlStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CrawlStatus::Idle     => write!(f, "idle"),
            CrawlStatus::Running  => write!(f, "running"),
            CrawlStatus::Paused   => write!(f, "paused"),
            CrawlStatus::Complete => write!(f, "complete"),
        }
    }
}