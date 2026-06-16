use tauri::State;
use crate::AppState;
use crate::models::WishlistItem;
use crate::error::AppError;

/// Fetch the user's Steam wishlist and store it locally.
/// Returns the list of wishlist items with current prices.
///
/// Requires steam_id to be set in settings first.
#[tauri::command]
pub async fn fetch_wishlist(
    state: State<'_, AppState>,
) -> Result<Vec<WishlistItem>, AppError> {
    // Load Steam ID from settings
    let settings = state.settings.load().await?;

    let steam_id = settings.steam_id.ok_or_else(|| {
        AppError::NotFound(
            "Steam ID not set. Please add your Steam ID in Settings.".to_string()
        )
    })?;

     let api_key = settings.steam_api_key.ok_or_else(||
        AppError::NotFound("Steam API key not set. Add it in Settings.".to_string())
    )?;

    // TODO: make configurable later
    let country_code = "lk";

    println!("[INFO] Fetching wishlist for Steam ID: {}", steam_id);

    // Fetch from Steam
    let raw_items = state.steam
        .get_full_wishlist(
            &steam_id, 
            &api_key, 
            &country_code,
            &state.limits.steam_store
        ).await?;

    let mut wishlist_items: Vec<WishlistItem> = Vec::new();

    for (app_id, date_added, details) in &raw_items {
        // Extract price from app details
        let current_price = details.price_overview
            .as_ref()
            .map(|p| p.final_price);

        let original_price = details.price_overview
            .as_ref()
            .map(|p| p.initial);

        let discount_percent = details.price_overview
            .as_ref()
            .map(|p| p.discount_percent);

        let header_image = Some(details.header_image.clone())
            .filter(|s| !s.is_empty());

        let short_description = Some(details.short_description.clone())
            .filter(|s| !s.is_empty());

        let item = WishlistItem {
            app_id: *app_id,
            name: details.name.clone(),
            review_summary: None,       // not in appdetails basic filter
            reviews_percent: None,
            reviews_total: None,
            date_added: Some(*date_added),
            current_price,
            original_price,
            historical_low: None,
            discount_percent,
            buy_recommendation: None,
            header_image,
            short_description
        };

        // Save to local DB
        state.wishlist.upsert(&item).await?;
        wishlist_items.push(item);
    }

    // Now enrich with app details (header images, genres) in background
    // We do this async but don't block the command return
    // 🦀 RUST LESSON: tokio::spawn for fire-and-forget background work
    // We can't pass `state` into a spawned task directly (lifetime issues)
    // so we extract what we need before spawning.
    // For now we return the basic items immediately and enrichment
    // happens on the next fetch. This keeps the UI responsive.

    println!("[INFO] Wishlist saved to DB, returning {} items", wishlist_items.len());
    Ok(wishlist_items)
}

/// Get wishlist from local DB (fast, no network).
/// Call this first — use fetch_wishlist to refresh from Steam.
#[tauri::command]
pub async fn get_wishlist(
    state: State<'_, AppState>,
) -> Result<Vec<WishlistItem>, AppError> {
    state.wishlist.get_all().await
}

/// Remove a game from the local wishlist cache.
#[tauri::command]
pub async fn remove_from_wishlist(
    state: State<'_, AppState>,
    app_id: i64,
) -> Result<(), AppError> {
    state.wishlist.delete(app_id).await
}

/// Enrich wishlist games with ITAD price data.
/// Updates historical_low in DB and returns updated wishlist.
/// Call this after fetch_wishlist to get the full price picture.
#[tauri::command]
pub async fn enrich_wishlist_prices(
    state: State<'_, AppState>,
) -> Result<Vec<WishlistItem>, AppError> {
    let settings = state.settings.load().await?;

    let itad_key = settings.itad_api_key.ok_or_else(||
        AppError::NotFound(
            "ITAD API key not set. Add it in Settings to see historical prices.".to_string()
        )
    )?;

    // Load wishlist from DB
    let wishlist = state.wishlist.get_all().await?;
    if wishlist.is_empty() {
        return Ok(vec![]);
    }

    let app_ids: Vec<i64> = wishlist.iter().map(|w| w.app_id).collect();

    // Country code for regional pricing — use "lk" for Sri Lanka
    // We'll make this configurable later
    let country = "lk";

    println!("[ITAD] Enriching {} wishlist games...", app_ids.len());

    let price_data = state.itad
        .enrich_games(&itad_key, &app_ids, country, &state.limits.itad)
        .await?;

    println!("[ITAD] Got price data for {} games", price_data.len());

    // Update each game in the DB with the historical low
    for data in &price_data {
        // Update historical_low in wishlist table
        if let Some(low) = data.historical_low {
            // Find the matching wishlist item and update it
            if let Some(item) = wishlist.iter().find(|w| w.app_id == data.steam_app_id) {
                let mut updated = item.clone();
                updated.historical_low = Some(low);

                // If current price <= historical low, it's at its lowest
                // Update discount if we have better data from ITAD
                if let Some(steam_cut) = data.steam_cut {
                    if steam_cut > 0 {
                        updated.discount_percent = Some(steam_cut);
                    }
                }

                state.wishlist.upsert(&updated).await?;
            }
        }

        // Record current price in price_history for future trend analysis
        if let Some(current) = data.steam_current {
            use chrono::Utc;
            use crate::models::PricePoint;

            let point = PricePoint {
                app_id: data.steam_app_id,
                price: current,
                discount_percent: data.steam_cut.unwrap_or(0),
                recorded_at: Utc::now(),
                source: "itad_steam".to_string(),
            };
            // Don't fail enrichment if price history insert fails
            let _ = state.prices.insert(&point).await;
        }
    }

    // Return freshly updated wishlist from DB
    state.wishlist.get_all().await
}