use crate::api::rate_limiter::RateLimiter;
use crate::error::{AppError, Result};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

// ─────────────────────────────────────────────
// API RESPONSE TYPES
// These match the exact JSON shape SteamSpy returns.
// Note: price fields are STRINGS in the API ("2249"), not numbers.
// ─────────────────────────────────────────────

/// One game entry as returned by SteamSpy.
/// Field names must match the JSON keys exactly — serde maps them 1:1.
#[derive(Debug, Clone, Deserialize)]
pub struct SteamSpyGame {
    pub appid: i64,
    pub name: String,

    pub developer: Option<String>,
    pub publisher: Option<String>,

    /// Positive review count
    pub positive: i64,
    /// Negative review count
    pub negative: i64,

    /// Owners range string e.g. "1,000,000 .. 2,000,000"
    pub owners: Option<String>,

    /// Average playtime since 2009, in minutes
    pub average_forever: Option<i64>,
    /// Average playtime last 2 weeks, in minutes
    pub average_2weeks: Option<i64>,

    // 🦀 RUST LESSON: #[serde(rename)]
    // The JSON key is "initialprice" but we want a clearer name in Rust.
    // serde(rename) maps the JSON key to our chosen field name.
    // This is like [JsonPropertyName] in C# System.Text.Json.
    /// Current price as a STRING in cents e.g. "999" = $9.99, "0" = Free
    #[serde(default)]
    pub price: Option<String>,

    /// Original price before discount, as a STRING in cents
    #[serde(rename = "initialprice", default)]
    pub initial_price: Option<String>,

    /// Current discount percentage as a STRING e.g. "75"
    #[serde(default)]
    pub discount: Option<String>,

    /// Peak concurrent users yesterday
    pub ccu: Option<i64>,

    /// Tags as a map of tag_name → vote_count
    /// e.g. {"Indie": 1500, "RPG": 1200, "Singleplayer": 900}
    pub tags: Option<HashMap<String, i64>>,

    pub genre: Option<String>,
    pub languages: Option<String>,
}

impl SteamSpyGame {
    /// Calculate the review score percentage (0.0 - 100.0)
    pub fn review_score(&self) -> f64 {
        let total = self.positive + self.negative;
        if total == 0 {
            return 0.0;
        }
        (self.positive as f64 / total as f64) * 100.0
    }

    /// Parse the price string to cents (i64)
    /// Returns None if the string is empty or "0" and it's the initial price
    pub fn price_cents(&self) -> Option<i64> {
        self.price
            .as_deref()
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse::<i64>().ok())
            .filter(|&p| p > 0)
    }

    pub fn initial_price_cents(&self) -> Option<i64> {
        self.initial_price
            .as_deref()
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse::<i64>().ok())
            .filter(|&p| p > 0)
    }

    pub fn discount_percent(&self) -> i64 {
        self.discount
            .as_deref()
            .filter(|s| !s.is_empty())
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0)
    }

    /// Returns true if the game has the "Indie" tag with meaningful votes
    pub fn is_indie(&self) -> bool {
        self.tags
            .as_ref()
            .and_then(|tags| tags.get("Indie"))
            .map(|&votes| votes > 10)
            .unwrap_or(false)
        // 🦀 RUST LESSON: method chaining on Option
        // .as_ref()          → &Option<HashMap> to Option<&HashMap> (borrow, don't move)
        // .and_then(...)     → if Some(tags), look up "Indie" key → Option<&i64>
        // .map(|&votes| ...) → if Some(votes), check the vote count → Option<bool>
        // .unwrap_or(false)  → if None at any point, return false
    }

    /// Top tags sorted by vote count, returns up to `limit` tag names
    pub fn top_tags(&self, limit: usize) -> Vec<String> {
        let Some(tags) = &self.tags else {
            return vec![];
        };
        // 🦀 RUST LESSON: let-else
        // `let Some(x) = expr else { return fallback; }` is a clean way to
        // handle the None case and return early without deep nesting.
        // It's like a guard clause in C#.

        let mut sorted: Vec<(&String, &i64)> = tags.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1)); // sort descending by vote count

        sorted
            .into_iter()
            .take(limit)
            .map(|(tag, _)| tag.clone())
            .collect()
    }
}

// SteamSpy returns a HashMap<String, SteamSpyGame> for list endpoints
// where the key is the appid as a string e.g. {"730": {...}, "440": {...}}
pub type SteamSpyList = HashMap<String, SteamSpyGame>;

// ─────────────────────────────────────────────
// THE CLIENT
// ─────────────────────────────────────────────
pub struct SteamSpyClient {
    // 🦀 RUST LESSON: Reusing the HTTP client
    // reqwest::Client manages a connection pool internally.
    // Creating one per request is wasteful — same as HttpClient in C#.
    // We store it in the struct and reuse it for every request.
    client: Client,
    base_url: String,
}

impl SteamSpyClient {
    pub fn new() -> Self {
        SteamSpyClient {
            client: Client::builder()
                // Identify ourselves politely — good API citizenship
                .user_agent("steam-lite/0.1 (github.com/gihan-aj/steam-lite)")
                .timeout(std::time::Duration::from_secs(15))
                .build()
                // .build() only fails for invalid TLS config — safe to unwrap here
                .expect("Failed to build HTTP client"),
            base_url: "https://steamspy.com/api.php".to_string(),
        }
    }

    /// Fetch the top 100 most-played games in the last two weeks.
    /// Great for discovering what's popular right now.
    pub async fn get_top100_in_2weeks(&self, limiter: &RateLimiter) -> Result<Vec<SteamSpyGame>> {
        self.fetch_list("top100in2weeks", &[], limiter).await
    }

    /// Fetch the top 100 most-played games of all time.
    pub async fn get_top100_forever(&self, limiter: &RateLimiter) -> Result<Vec<SteamSpyGame>> {
        self.fetch_list("top100forever", &[], limiter).await
    }

    /// Fetch detailed data for a single game by its Steam App ID.
    pub async fn get_app_details(&self, app_id: i64) -> Result<SteamSpyGame> {
        let app_id_str = app_id.to_string();
        let params = &[("request", "appdetails"), ("appid", &app_id_str)];

        let response = self
            .client
            .get(&self.base_url)
            .query(params)
            .send()
            .await
            .map_err(|e| AppError::Api(format!("SteamSpy request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "SteamSpy returned status {}",
                response.status()
            )));
        }

        response
            .json::<SteamSpyGame>()
            .await
            .map_err(|e| AppError::Parse(format!("Failed to parse SteamSpy game: {}", e)))
    }

    /// Fetch one page of all games on Steam (1000 per page, starts at page 0).
    /// Rate limit: 1 request per MINUTE for this endpoint.
    pub async fn get_all_page(
        &self,
        page: u32,
        limiter: &RateLimiter,
    ) -> Result<Vec<SteamSpyGame>> {
        let page_str = page.to_string();
        self.fetch_list("all", &[("page", &page_str)], limiter)
            .await
    }

    // ─────────────────────────────────────────────
    // Private helpers
    // ─────────────────────────────────────────────

    /// Internal helper: fetch a list endpoint and convert HashMap to Vec.
    ///
    /// 🦀 RUST LESSON: &[(&str, &str)] — a slice of string tuple references
    /// This lets callers pass extra query params without heap allocation.
    /// `&[]` means "no extra params".
    async fn fetch_list(
        &self,
        request_type: &str,
        extra_params: &[(&str, &str)],
        limiter: &RateLimiter,
    ) -> Result<Vec<SteamSpyGame>> {
        limiter.acquire().await;
        // Build query params: always include `request`, then any extras
        let mut params = vec![("request", request_type)];
        params.extend_from_slice(extra_params);

        let response = self
            .client
            .get(&self.base_url)
            .query(&params)
            .send()
            .await
            .map_err(|e| AppError::Api(format!("SteamSpy request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "SteamSpy returned HTTP {}",
                response.status()
            )));
        }

        let map = response
            .json::<SteamSpyList>()
            .await
            .map_err(|e| AppError::Parse(format!("Failed to parse SteamSpy response: {}", e)))?;

        // Convert HashMap values to a Vec, sorted by positive reviews descending
        let mut games: Vec<SteamSpyGame> = map.into_values().collect();
        games.sort_by(|a, b| b.positive.cmp(&a.positive));

        Ok(games)
    }
}
