// IsThereAnyDeal API v2 client
//
// The flow for enriching wishlist games:
//   1. Lookup each Steam app_id → get ITAD UUID (per game, GET)
//   2. Batch fetch historical lows (up to 200 at once, POST)
//   3. Batch fetch current prices from Steam (up to 200 at once, POST)
//
// 🦀 RUST LESSON: POST requests with JSON body
// So far we've only done GET requests. ITAD's batch endpoints use POST
// with a JSON array body. reqwest handles this with .json(&body).
// The body is serialized automatically via serde.

use reqwest::Client;
use serde::{Deserialize};
use crate::error::{AppError, Result};
use crate::api::rate_limiter::{RateLimiter};

// ─────────────────────────────────────────────
// RESPONSE TYPES
// ─────────────────────────────────────────────

/// Response from /games/lookup/v1?appid={id}
#[derive(Debug, Deserialize)]
pub struct LookupResponse {
    pub found: bool,
    pub game: Option<ItadGame>,
}

#[derive(Debug, Deserialize)]
pub struct ItadGame {
    /// ITAD's internal UUID for this game — used for all other endpoints
    pub id: String,
    // pub slug: String,
    // pub title: String,
}

/// One item from the /games/historylow/v1 response
#[derive(Debug, Deserialize)]
pub struct HistoryLowItem {
    /// ITAD game UUID
    pub id: String,
    pub low: Option<HistoryLowPrice>,
}

#[derive(Debug, Deserialize)]
pub struct HistoryLowPrice {
    pub shop: Shop,
    pub price: Price,
    pub regular: Price,
    /// Discount percentage at time of historical low
    pub cut: i64,
    /// When this low was recorded
    pub timestamp: String,
}

/// One item from the /games/overview/v2 response
#[derive(Debug, Deserialize)]
pub struct OverviewItem {
    pub id: String,
    pub current: Option<CurrentDeal>,
    pub lowest: Option<LowestDeal>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentDeal {
    pub shop: Shop,
    pub price: Price,
    pub regular: Price,
    pub cut: i64,
}

#[derive(Debug, Deserialize)]
pub struct LowestDeal {
    pub shop: Shop,
    pub price: Price,
    pub cut: i64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct Shop {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Price {
    pub amount: f64,
    /// Price in integer cents — e.g. 999 = $9.99
    #[serde(rename = "amountInt")]
    pub amount_int: i64,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
pub struct GameInfo {
    pub id:          String,
    pub tags:        Option<Vec<String>>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    pub developers:  Option<Vec<InfoDeveloper>>,
    pub reviews:     Option<Vec<ReviewScore>>,
}

#[derive(Debug, Deserialize)]
pub struct InfoDeveloper {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ReviewScore {
    pub score:  i64,
    pub source: String,   // "Steam", "Metascore", "OpenCritic", etc.
    pub count:  Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PriceHistoryEntry {
    pub timestamp: String,
    pub deal: HistoryDeal,
}

#[derive(Debug, Deserialize)]
pub struct HistoryDeal {
    pub price:   Price,
    pub regular: Price,
    pub cut:     i64,
}

// ─────────────────────────────────────────────
// ENRICHED RESULT — what we return to callers
// ─────────────────────────────────────────────

/// All ITAD data for one wishlist game, ready to store in DB
#[derive(Debug)]
pub struct GamePriceData {
    pub steam_app_id:        i64,
    pub itad_id:             String,
    /// All-time lowest price across all stores (cents)
    pub historical_low:      Option<i64>,
    /// All-time lowest % discount across all stores
    pub historical_low_cut:  Option<i64>,
    /// Store that had the historical low
    pub historical_low_shop: Option<String>,
    pub historical_low_timestamp: Option<String>,
    /// Steam's current lowest price (cents)
    pub steam_current:       Option<i64>,
    /// Current % cut on Steam
    pub steam_cut:           Option<i64>,
    /// Steam's all-time lowest price (cents)
    pub steam_low:           Option<i64>,
    /// Steam's all-time lowest % cut on Steam
    pub steam_low_cut:           Option<i64>,
    pub steam_low_timestamp:      Option<String>,
}

// ─────────────────────────────────────────────
// THE CLIENT
// ─────────────────────────────────────────────

pub struct ItadClient {
    client: Client,
    base_url: String,
}

impl ItadClient  {
    pub fn new() -> Self {
        ItadClient {
            client: Client::builder()
                .user_agent("steam-lite/0.1 (+https://github.com/gihan-aj/steam-lite)")
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .expect("Failed to build ITAD HTTP client"),
            base_url: "https://api.isthereanydeal.com".to_string(),
        }
    }

    /// Step 1: Look up the ITAD UUID for a Steam app_id.
    /// Returns None if ITAD doesn't know this game.
    pub async fn lookup_by_steam_appid(
        &self,
        api_key: &str,
        steam_appid: i64,
        limiter: &RateLimiter,
    ) -> Result<Option<String>> {
        limiter.acquire().await;

        let url = format!("{}/games/lookup/v1", self.base_url);
        let response = self.client
            .get(&url)
            .query(&[
                ("key", api_key),
                ("appid", &steam_appid.to_string())
            ])
            .send()
            .await
            .map_err(|e| AppError::Api(format!("ITAD lookup failed: {}", e)))?;
        
        if response.status() == 404 {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "ITAD lookup returned {}", response.status()
            )));
        }

        let data: LookupResponse = response
            .json()
            .await
            .map_err(|e| AppError::Parse(format!("Failed to parse ITAD lookup: {}", e)))?;

        if !data.found {
            return Ok(None);
        }

        Ok(data.game.map(|g| g.id))
    }

    /// Step 2: Batch fetch historical lows for up to 200 ITAD game IDs.
    ///
    /// 🦀 RUST LESSON: POST with JSON body
    /// `.json(&body)` serializes the body and sets Content-Type: application/json
    /// The `&[String]` slice reference avoids cloning the whole Vec.
    pub async fn get_history_lows(
        &self,
        api_key: &str,
        itad_ids: &[String],
        country: &str,
        limiter: &RateLimiter,
    ) -> Result<Vec<HistoryLowItem>> {
        limiter.acquire().await;

        let url = format!("{}/games/historylow/v1", self.base_url);

        let response = self.client
            .post(&url)
            .query(&[("key", api_key), ("country", country)])
            .json(itad_ids)
            .send()
            .await
            .map_err(|e| AppError::Api(format!("ITAD historylow failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "ITAD historylow returned {}", response.status()
            )));
        }

        response
            .json::<Vec<HistoryLowItem>>()
            .await
            .map_err(|e| AppError::Parse(format!("Failed to parse history lows: {}", e)))
    }

    /// Step 3: Batch fetch current prices + store lows for up to 200 ITAD game IDs.
    /// shop 61 = Steam specifically.
    pub async fn get_price_overview(
        &self,
        api_key: &str,
        itad_ids: &[String],
        country: &str,
        limiter: &RateLimiter,
    ) -> Result<Vec<OverviewItem>> {
        limiter.acquire().await;

        let url = format!("{}/games/overview/v2", self.base_url);
        let response = self.client
            .post(&url)
            .query(&[
                ("key", api_key),
                ("country", country),
                ("shops", "61"),   // Steam only
            ])
            .json(itad_ids)
            .send()
            .await
            .map_err(|e| AppError::Api(format!("ITAD overview failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "ITAD overview returned {}", response.status()
            )));
        }

        // Overview wraps the list in a { "prices": [...], "bundles": [...] } object
        #[derive(Deserialize)]
        struct OverviewResponse {
            prices: Vec<OverviewItem>,
        }

        let data: OverviewResponse = response
            .json()
            .await
            .map_err(|e| AppError::Parse(format!("Failed to parse price overview: {}", e)))?;

        Ok(data.prices)
    }

    /// Full enrichment: takes a list of (steam_app_id) values,
    /// returns GamePriceData for each one that ITAD knows about.
    ///
    /// This is the main method called from the command layer.
    pub async fn enrich_games(
        &self,
        api_key: &str,
        steam_app_ids: &[i64],
        country: &str,
        limiter: &RateLimiter,
    ) -> Result<Vec<GamePriceData>>{
        if steam_app_ids.is_empty() {
            return Ok(vec![]);
        }

        tracing::debug!(count = steam_app_ids.len(), "ITAD: resolving Steam app IDs");

        // Step 1: Resolve each Steam app_id to an ITAD UUID
        // We do these sequentially to respect rate limits
        let mut id_pairs: Vec<(i64, String)> = Vec::new();  // (steam_id, itad_uuid)

        for &app_id in steam_app_ids {
            match self.lookup_by_steam_appid(api_key, app_id, limiter).await {
                Ok(Some(itad_id)) => {
                    id_pairs.push((app_id, itad_id));
                }
                Ok(None) => {
                    tracing::warn!(app_id, "Game not found in ITAD");
                }
                Err(e) => {
                    // Don't fail the whole batch if one lookup fails
                    tracing::warn!(app_id, error = %e, "ITAD lookup failed");
                }
            }
        }

        if id_pairs.is_empty() {
            return Ok(vec![]);
        }

        tracing::info!(count = id_pairs.len(), "ITAD: IDs resolved, fetching price data");

        // Collect just the ITAD UUIDs for the batch calls
        let itad_ids: Vec<String> = id_pairs.iter().map(|(_, id)| id.clone()).collect();

        // Step 2 & 3: Batch fetch (single call each, up to 200 games)
        let history_lows = self
            .get_history_lows(api_key, &itad_ids, country, limiter)
            .await
            .unwrap_or_default();  // Don't fail if this endpoint errors

        let overviews = self
            .get_price_overview(api_key, &itad_ids, country, limiter)
            .await
            .unwrap_or_default();

        // Build lookup maps: itad_id → data
        // 🦀 RUST LESSON: HashMap::into_iter().collect() for transformations
        use std::collections::HashMap;

        let low_map: HashMap<String, HistoryLowItem> = history_lows
            .into_iter()
            .map(|item| (item.id.clone(), item))
            .collect();

        let overview_map: HashMap<String, OverviewItem> = overviews
            .into_iter()
            .map(|item| (item.id.clone(), item))
            .collect();

        // Assemble final results
        let results = id_pairs
            .into_iter()
            .map(|(steam_app_id, itad_id)| {
                let low = low_map.get(&itad_id);
                let overview = overview_map.get(&itad_id);

                GamePriceData {
                    steam_app_id,
                    itad_id: itad_id.clone(),

                    historical_low: low
                        .and_then(|l| l.low.as_ref())
                        .map(|l| l.price.amount_int),

                    historical_low_cut: low
                        .and_then(|l| l.low.as_ref())
                        .map(|l| l.cut),

                    historical_low_shop: low
                        .and_then(|l| l.low.as_ref())
                        .map(|l| l.shop.name.clone()),

                    historical_low_timestamp: low
                        .and_then(|l| l.low.as_ref())
                        .map(|l| l.timestamp.clone()),

                    steam_current: overview
                        .and_then(|o| o.current.as_ref())
                        .map(|c| c.price.amount_int),

                    steam_cut: overview
                        .and_then(|o| o.current.as_ref())
                        .map(|c| c.cut),

                    steam_low: overview
                        .and_then(|o| o.lowest.as_ref())
                        .map(|l| l.price.amount_int),

                    steam_low_cut: overview
                        .and_then(|o| o.lowest.as_ref())
                        .map(|c| c.cut),
                    
                    steam_low_timestamp: overview
                        .and_then(|o| o.lowest.as_ref())
                        .map(|l| l.timestamp.clone()),
                }
            })
            .collect();

        Ok(results)
    }

    /// Fetch rich game info: tags, release date, review scores, developers.
    /// Called once per game during enrichment — result stored in DB.
    pub async fn get_game_info(
        &self,
        api_key: &str,
        itad_id: &str,
        limiter: &RateLimiter,
    ) -> Result<GameInfo> {
        limiter.acquire().await;

        let url = format!("{}/games/info/v2", self.base_url);
        let response = self.client
            .get(&url)
            .query(&[("key", api_key), ("id", itad_id)])
            .send()
            .await
            .map_err(|e| AppError::Api(format!("ITAD info failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "ITAD info returned {}", response.status()
            )));
        }

        response.json::<GameInfo>()
            .await
            .map_err(|e| AppError::Parse(format!("Failed to parse game info: {}", e)))
    }

    /// Fetch Steam price history for a game, filtered to Steam only.
    /// Returns entries sorted oldest → newest.
    /// `since` is ISO 8601 e.g. "2024-01-01T00:00:00Z"
    pub async fn get_price_history(
        &self,
        api_key: &str,
        itad_id: &str,
        since: &str,
        limiter: &RateLimiter,
    ) -> Result<Vec<PriceHistoryEntry>> {
        limiter.acquire().await;

        let url = format!("{}/games/history/v2", self.base_url);
        let response = self.client
            .get(&url)
            .query(&[
                ("key",   api_key),
                ("id",    itad_id),
                ("shops", "61"),       // Steam only
                ("since", since),
            ])
            .send()
            .await
            .map_err(|e| AppError::Api(format!("ITAD history failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "ITAD history returned {}", response.status()
            )));
        }

        let mut entries: Vec<PriceHistoryEntry> = response
            .json()
            .await
            .map_err(|e| AppError::Parse(format!("Failed to parse price history: {}", e)))?;

        // Sort oldest first — better for chart rendering
        entries.sort_by_key(|e| e.timestamp.clone());
        Ok(entries)
    }

 }