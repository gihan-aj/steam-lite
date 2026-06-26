use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::error::{AppError, Result};
use crate::api::rate_limiter::RateLimiter;

// 🦀 RUST LESSON: #[serde(default)]
// Steam's API is inconsistent — some fields exist on some games but not others.
// `#[serde(default)]` means: if this field is missing from the JSON,
// use the type's Default value (None for Option, false for bool, 0 for numbers).
// Without it, serde returns an error if any field is absent.

// ─────────────────────────────────────────────
// WISHLIST TYPES
// ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct WishlistApiResponse {
    response: WishlistApiItems,
}

#[derive(Debug, Deserialize)]
struct WishlistApiItems {
    #[serde(default)]
    items: Vec<WishlistApiItem>,
}

#[derive(Debug, Deserialize)]
pub struct WishlistApiItem {
    pub appid: i64,
    #[serde(default)]
    priority: i64,
    #[serde(default)]
    pub date_added: i64,
}


// ─────────────────────────────────────────────
// APP DETAILS TYPES
// ─────────────────────────────────────────────

/// Response from store.steampowered.com/api/appdetails
/// Each app_id gets its own wrapper object.
#[derive(Debug, Deserialize)]
pub struct AppDetailsResponse {
    pub success: bool,
    pub data: Option<AppDetails>,
}

#[derive(Debug, Deserialize)]
pub struct AppDetails {
    pub name: String,

    #[serde(default)]
    pub short_description: String,

    /// The 460x215 header image — main game banner
    #[serde(default)]
    pub header_image: String,

    /// Small 231x87 capsule image
    #[serde(default)]
    pub capsule_image: String,

    #[serde(default)]
    pub genres: Vec<Genre>,

    #[serde(default)]
    pub categories: Vec<Category>,

    pub price_overview: Option<PriceOverview>,

    #[serde(default)]
    pub developers: Vec<String>,

    #[serde(default)]
    pub publishers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Genre {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: i64,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceOverview {
    pub currency: String,

    /// Price in cents e.g. 999 = $9.99
    pub initial: i64,
    #[serde(rename = "final")]
    pub final_price: i64,    // renamed from `final` — reserved keyword in Rust!
    pub discount_percent: i64,

    /// Formatted price string e.g. "$9.99"
    pub initial_formatted: String,
    pub final_formatted: String,
}

impl AppDetails {
    pub fn is_indie(&self) -> bool {
        self.genres.iter().any(|g| g.description == "Indie")
    }
}

// ─────────────────────────────────────────────
// THE CLIENT
// ─────────────────────────────────────────────
pub struct SteamClient {
    client: Client,
}

impl SteamClient{
    pub fn new() -> Self {
        SteamClient {
            client: Client::builder()
                .user_agent("steam-lite/0.1 (github.com/gihan-aj/steam-lite)")
                .timeout(std::time::Duration::from_secs(20))
                .build()
                .expect("Failed to build Steam HTTP client"),
        }
    }

    /// Fetch wishlist using the new official Steam API.
    /// Returns Vec of (app_id, priority, date_added).
    /// Requires a Steam API key.
    pub async fn get_wishlist_ids(
        &self,
        steam_id: &str,
        api_key: &str
    ) -> Result<Vec<WishlistApiItem>>{
        let url = format!(
            "https://api.steampowered.com/IWishlistService/GetWishlist/v1/?key={}&steamid={}",
            api_key, steam_id
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Api(format!("Wishlist fetch failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "Steam API returned {} — check your API key and Steam ID",
                response.status()
            )));
        }

        let data: WishlistApiResponse = response
            .json()
            .await
            .map_err(|e| AppError::Parse(format!("Failed to parse wishlist: {}", e)))?;

        Ok(data.response.items)
    }

    /// Full wishlist fetch: get IDs, then enrich each with app details.
    /// Batches appdetails calls to respect rate limits.
    pub async fn get_full_wishlist(
        &self,
        steam_id: &str,
        api_key: &str,
        country_code: &str,
        limiter: &RateLimiter,
    ) -> Result<Vec<(i64, i64, AppDetails)>>{
        limiter.acquire().await;
        // Get all wishlist app IDs
        let mut items = self.get_wishlist_ids(steam_id, api_key).await?;
        // Sort by priority (user's wishlist order)
        items.sort_by_key(|i| i.priority);

        println!("[INFO] Wishlist has {} items, fetching details...", items.len());

        let mut result = Vec::new();

        // Fetch app details for each game
        // We do them one at a time with a small delay to be respectful of rate limits
        // For a wishlist of ~50 games this takes ~10 seconds total
        for item in &items {
            // 🦀 Acquire a rate limit token BEFORE each API call
            // If we're going too fast, this will sleep automatically
            limiter.acquire().await;

            match self.get_app_details(item.appid, country_code).await {
                Ok(Some(details)) => {
                    result.push((item.appid, item.date_added, details));
                }
                Ok(None) => {
                    // Game may have been removed from Steam
                    println!("[WARN] No details for app_id {}", item.appid);
                }
                Err(e) => {
                    // Don't fail the whole wishlist if one game fails
                    println!("[WARN] Failed to get details for {}: {}", item.appid, e);
                }
            }
        }

        println!("[INFO] Got details for {}/{} wishlist games", result.len(), items.len());
        Ok(result)
    }

    /// Fetch app details for a single game.
    /// `country_code` e.g. "us", "lk", "gb" for regional pricing.
    pub async fn get_app_details(
        &self,
        app_id: i64,
        country_code: &str,
    ) -> Result<Option<AppDetails>> {
        // Note: caller is responsible for rate limiting
        // get_app_details itself is intentionally dumb — just HTTP
        let url = format!(
            "https://store.steampowered.com/api/appdetails?appids={}&cc={}&filters=basic,genres,price_overview,categories",
            app_id, country_code
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Api(format!("App details fetch failed: {}", e)))?;

        // Response is HashMap<String, AppDetailsResponse>
        // e.g. {"1245620": {"success": true, "data": {...}}}
        let mut map: HashMap<String, AppDetailsResponse> = response
            .json()
            .await
            .map_err(|e| AppError::Parse(format!("Failed to parse app details: {}", e)))?;

        // 🦀 RUST LESSON: .remove() on HashMap
        // We use remove() instead of get() to take ownership of the value,
        // avoiding a clone. The map only has one entry anyway.
        let app_id_str = app_id.to_string();
        let entry = map.remove(&app_id_str);

        Ok(entry
            .filter(|e| e.success)
            .and_then(|e| e.data))
    }
}