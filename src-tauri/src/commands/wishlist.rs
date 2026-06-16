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
    let raw_items = state.steam.get_full_wishlist(&steam_id, &api_key, &country_code).await?;

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
        
        let item = WishlistItem {
            app_id: *app_id,
            name: details.name.clone(),
            review_summary: None,       // not in appdetails basic filter
            reviews_percent: None,
            reviews_total: None,
            date_added: Some(*date_added),
            current_price,
            historical_low: None,
            discount_percent,
            buy_recommendation: None,
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