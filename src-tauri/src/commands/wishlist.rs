use tauri::State;
use crate::{AppState};
use crate::models::{WishlistItem};
use crate::error::AppError;
use crate::services::price_intelligence::{
    compute_predicted_regional_low,
    is_at_regional_low,
    compute_price_signal,
    compute_itad_discrepancy,
    compute_review_label
};
use crate::services::sale_analyzer::{analyze_sale_pattern, PricePoint as SalePoint};
use chrono::{DateTime, Duration, Utc};

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

    let country_code = &settings.country_code.as_str();

    println!("[INFO] Fetching wishlist for Steam ID: {}", steam_id);

    // Fetch from Steam
    let raw_items = state.steam
        .get_full_wishlist(
            &steam_id, 
            &api_key, 
            &country_code,
            &state.limits.steam_store
        ).await?;

    let fetched_ids: Vec<i64> = raw_items.iter().map(|(id, _, _)| *id).collect();

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

        let existing = state.wishlist.get_by_id(*app_id).await?;

        let item = WishlistItem {
            app_id: *app_id,
            name: details.name.clone(),
            date_added: Some(*date_added),
            current_price,
            original_price,
            discount_percent,
            header_image,
            short_description,

            // ── Carry forward enriched fields if they exist ──────────
            // These come from ITAD enrichment — don't overwrite with None
            review_summary:         existing.as_ref().and_then(|e| e.review_summary.clone()),
            reviews_percent:        existing.as_ref().and_then(|e| e.reviews_percent),
            reviews_total:          existing.as_ref().and_then(|e| e.reviews_total),
            historical_low:         existing.as_ref().and_then(|e| e.historical_low),
            steam_historical_cut:   existing.as_ref().and_then(|e| e.steam_historical_cut),
            steam_historical_date:  existing.as_ref().and_then(|e| e.steam_historical_date.clone()),
            all_time_low_cut:       existing.as_ref().and_then(|e| e.all_time_low_cut),
            all_time_low_shop:      existing.as_ref().and_then(|e| e.all_time_low_shop.clone()),
            all_time_low_date:      existing.as_ref().and_then(|e| e.all_time_low_date.clone()),
            predicted_regional_low: existing.as_ref().and_then(|e| e.predicted_regional_low),
            is_at_regional_low:     existing.as_ref().map(|e| e.is_at_regional_low).unwrap_or(false),
            steam_review_score:     existing.as_ref().and_then(|e| e.steam_review_score),
            steam_review_count:     existing.as_ref().and_then(|e| e.steam_review_count),
            review_label:           existing.as_ref().and_then(|e| e.review_label.clone()),
            opencritic_score:       existing.as_ref().and_then(|e| e.opencritic_score),
            metacritic_score:       existing.as_ref().and_then(|e| e.metacritic_score),
            release_date:           existing.as_ref().and_then(|e| e.release_date.clone()),
            tags:                   existing.as_ref().map(|e| e.tags.clone()).unwrap_or_default(),
            developers:             existing.as_ref().map(|e| e.developers.clone()).unwrap_or_default(),
            itad_id:                existing.as_ref().and_then(|e| e.itad_id.clone()),
            avg_sale_interval_days: existing.as_ref().and_then(|e| e.avg_sale_interval_days),
            typical_discount_min:   existing.as_ref().and_then(|e| e.typical_discount_min),
            typical_discount_max:   existing.as_ref().and_then(|e| e.typical_discount_max),
            last_sale_date:         existing.as_ref().and_then(|e| e.last_sale_date.clone()),
            predicted_next_sale:    existing.as_ref().and_then(|e| e.predicted_next_sale.clone()),

            // ── Computed fields — always None here, attached below ───
            buy_recommendation: None,
            price_signal:       None,
            itad_discrepancy:   None,
        };

        let mut item = item;
        item.predicted_regional_low = compute_predicted_regional_low(&item);
        item.is_at_regional_low     = is_at_regional_low(&item);
        item.historical_low         = item.predicted_regional_low;

        // Save to local DB
        state.wishlist.upsert(&item).await?;
        wishlist_items.push(item);
    }

    match state.wishlist.delete_removed(&fetched_ids).await {
        Ok(0)       => println!("[INFO] Wishlist sync: no removed games"),
        Ok(removed) => println!("[INFO] Wishlist sync: removed {} games no longer wishlisted", removed),
        Err(e)      => println!("[WARN] Failed to clean up removed wishlist games: {}", e),
        // Don't fail the whole sync for a cleanup error
    }

    for item in &mut wishlist_items {
        let signal = compute_price_signal(item);
        item.price_signal = Some(signal);

        // We don't store US base price currently, so skip for now
        item.itad_discrepancy = compute_itad_discrepancy(item.current_price, None);
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
    // state.wishlist.get_all().await
    let mut wishlist = state.wishlist.get_all().await?;
    if wishlist.len() > 0 {
        for item in &mut wishlist {
            let signal = compute_price_signal(item);
            item.price_signal = Some(signal);

            // We don't store US base price currently, so skip for now
            item.itad_discrepancy = compute_itad_discrepancy(item.current_price, None);
        }
    }

    // Return freshly updated wishlist from DB
    // state.wishlist.get_all().await
    Ok(wishlist)
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
    let country = &settings.country_code.as_str();

    // Load wishlist from DB
    let wishlist = state.wishlist.get_all().await?;
    if wishlist.is_empty() {
        return Ok(vec![]);
    }

    let app_ids: Vec<i64> = wishlist.iter().map(|w| w.app_id).collect();

    println!("[ITAD] Enriching {} wishlist games...", app_ids.len());

    let price_data = state.itad
        .enrich_games(&itad_key, &app_ids, country, &state.limits.itad)
        .await?;

    println!("[ITAD] Got price data for {} games", price_data.len());

    // use std::collections::HashMap;
    // let itad_id_map : HashMap<i64, String> = price_data.iter()
    //     .map(|d| (d.steam_app_id, d.itad_id.clone()))
    //     .collect();

    // "since" = 2 years ago
    let since = (Utc::now() - Duration::days(730))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string(); 

    // Update each game in the DB with the historical low
    for data in &price_data {
        // Find the matching wishlist item and update it
        if let Some(item) = wishlist.iter().find(|w| w.app_id == data.steam_app_id) {
            let mut updated = item.clone();

            updated.steam_historical_cut = data.steam_low_cut;
            updated.steam_historical_date = data.steam_low_timestamp
                .as_ref()
                .and_then(|ts| ts.get(..7))  // "2026-06"
                .map(|s| s.to_string());

            if data.historical_low.is_some() {
                updated.all_time_low_cut  = data.historical_low_cut;
                updated.all_time_low_shop = data.historical_low_shop.clone();
                updated.all_time_low_date = data.historical_low
                    .as_ref()
                    .and_then(|_| data.historical_low_timestamp.as_ref())
                    .and_then(|ts| ts.get(..7))
                    .map(|s| s.to_string());
            }

            updated.predicted_regional_low = compute_predicted_regional_low(&updated);
            updated.is_at_regional_low = is_at_regional_low(&updated);
            updated.historical_low = updated.predicted_regional_low;

            updated.itad_id = Some(data.itad_id.clone());

            match state.itad.get_game_info(&itad_key, &data.itad_id, &state.limits.itad).await {
                Ok(game_info) => {
                    if let Some(reviews) = &game_info.reviews {
                        if let Some(steam_review) = reviews.iter().find(|r| r.source == "Steam") {
                            let count = steam_review.count.unwrap_or(0);
                            updated.steam_review_score = Some(steam_review.score);
                            updated.steam_review_count = Some(count);
                            updated.review_label = Some(
                                compute_review_label(steam_review.score, count).to_string()
                            );

                            // Backfill the old fields
                            updated.reviews_percent = Some(steam_review.score);
                            updated.reviews_total   = Some(count);
                            updated.review_summary  = updated.review_label.clone();
                        }
                        if let Some(oc) = reviews.iter().find(|r| r.source == "OpenCritic") {
                            updated.opencritic_score = Some(oc.score);
                        }
                        if let Some(mc) = reviews.iter().find(|r| r.source == "Metascore") {
                            updated.metacritic_score = Some(mc.score);
                        }
                    }

                    updated.release_date = game_info.release_date;

                    if let Some(tags) = game_info.tags {
                        updated.tags = tags;
                    }

                    if let Some(devs) = game_info.developers {
                        let names: Vec<String> = devs.iter().map(|d| d.name.clone()).collect();
                        updated.developers = names;
                    }
                }
                Err(e) => {
                    println!("[WARN] Failed to get info for {}: {}", updated.name, e);
                }
            }
            
            match state.itad.get_price_history(&itad_key, &data.itad_id, &since, &state.limits.itad).await {
                Ok(price_history) => {
                    if price_history.len() > 0 {
                        for entry in &price_history {
                            if let Ok(ts) = entry.timestamp.parse::<chrono::DateTime<Utc>>() {
                                let _ = state.prices.insert(&crate::models::PricePoint {
                                    app_id:           data.steam_app_id,
                                    price:            entry.deal.price.amount_int,
                                    discount_percent: entry.deal.cut,
                                    recorded_at:      ts,
                                    source:           "itad_history".to_string(),
                                }).await;
                            }
                        }

                        // Convert to SalePoints for analysis
                        let sale_points: Vec<SalePoint> = price_history.iter()
                            .filter_map(|e| {
                                e.timestamp.parse::<DateTime<Utc>>().ok()
                                    .map(|ts| SalePoint { timestamp: ts, cut: e.deal.cut })
                            })
                            .collect();

                        let pattern = analyze_sale_pattern(&sale_points);

                        updated.avg_sale_interval_days = pattern.avg_interval_days;
                        updated.typical_discount_min   = pattern.typical_min_cut;
                        updated.typical_discount_max   = pattern.typical_max_cut;
                        updated.last_sale_date         = pattern.last_sale_date;
                        updated.predicted_next_sale    = pattern.predicted_next;

                        println!(
                            "[ITAD] {} — {} sales, avg interval: {:?} days, next: {:?}",
                            updated.name, pattern.sale_count,
                            pattern.avg_interval_days, updated.predicted_next_sale
                        );
                    }
                }
                Err(e) => {
                    println!("[WARN] Failed to get history for {}: {}", updated.name, e);
                }
            }                     

            let signal = compute_price_signal(&updated);
            updated.price_signal = Some(signal);

            // println!(
            //     "[ITAD] {} — steam cut: {:?}%, regional base: {:?}, predicted low: {:?}, at low: {}",
            //     updated.name,
            //     updated.steam_historical_cut,
            //     updated.original_price.map(|p| format!("${:.2}", p as f64 / 100.0)),
            //     updated.predicted_regional_low.map(|p| format!("${:.2}", p as f64 / 100.0)),
            //     updated.is_at_regional_low,
            // );

            state.wishlist.upsert(&updated).await?;
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

    // get the wishlist from db
    let mut wishlist = state.wishlist.get_all().await?;
    if wishlist.len() > 0 {
        for item in &mut wishlist {
            let signal = compute_price_signal(item);
            item.price_signal = Some(signal);

            // We don't store US base price currently, so skip for now
            item.itad_discrepancy = compute_itad_discrepancy(item.current_price, None);
        }
    }

    // Return freshly updated wishlist from DB
    // state.wishlist.get_all().await
    Ok(wishlist)
}

/// Fetch stored price history for a specific game.
/// Used by the game detail panel to render the price chart.
#[tauri::command]
pub async fn get_game_price_history(
    state: State<'_, AppState>, 
    app_id: i64
) -> Result<Vec<crate::models::PricePoint>, AppError> {
  state.prices.get_history(app_id).await
}