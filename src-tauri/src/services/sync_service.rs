use chrono::{DateTime, Duration, Utc};

use crate::{ Result, AppState, models::PricePoint, services::price_intelligence::{compute_predicted_regional_low, compute_price_signal, is_at_regional_low}};

#[derive(Debug, Default)]
pub struct SyncResult {
    pub games_checked:  u32,
    pub prices_changed:  u32,
    pub games_added:    u32,
    pub games_removed:  u32,
    pub errors:         u32,          
}

// ─────────────────────────────────────────────
// CHECK IF SYNC IS DUE
// ─────────────────────────────────────────────

/// Returns true if enough time has elapsed since the last sync.
/// Called before every sync attempt.
pub async fn is_sync_due(state: &AppState) -> bool {
    let settings = match state.settings.load().await {
        Ok(s) => s,
        Err(_) => return  true,
    };

    settings.last_synced_at
        .map(|last| {
            let elapsed = Utc::now() - last;
            elapsed.num_hours() >= settings.sync_interval_hours
        })
        .unwrap_or(true) // never synced -> always due
}

/// Returns true if the app has been offline longer than the gap threshold.
/// Gap threshold = 2.5× the sync interval.
pub async fn needs_gap_fill(state: &AppState) -> bool {
    let settings = match state.settings.load().await {
        Ok(s) => s,
        Err(_) => return false,
    };

    let gap_threshold = Duration::hours(settings.sync_interval_hours * 5 / 2);

    settings.last_synced_at
        .map(|last| Utc::now() - last > gap_threshold)
        .unwrap_or(false)
}

// ─────────────────────────────────────────────
// LIGHT PRICE REFRESH
// Steam appdetails only — no ITAD calls.
// Checks for price changes on all wishlist games.
// ─────────────────────────────────────────────

pub async fn refresh_prices(state: &AppState) -> Result<SyncResult> {
    let settings = state.settings.load().await?;
    let country = settings.country_code.clone();

    let wishlist = state.wishlist.get_all().await?;
    let mut result = SyncResult::default();

    if wishlist.is_empty() {
        return Ok(result);
    }

    for item in wishlist {
        state.limits.steam_store.acquire().await;

        match state.steam.get_app_details(item.app_id, &country).await {
            Ok(Some(details)) => {
                result.games_checked += 1;

                let new_price    = details.price_overview.as_ref().map(|p| p.final_price);
                let new_discount = details.price_overview.as_ref().map(|p| p.discount_percent);
                let new_original = details.price_overview.as_ref().map(|p| p.initial);
    
                let price_changed = new_price   != item.current_price
                                || new_discount != item.discount_percent;

                if price_changed {
                    if let Some(price) = new_price {
                        let _ = state.prices.insert(&PricePoint {
                            app_id:             item.app_id,
                            price,
                            discount_percent:   new_discount.unwrap_or(0),
                            recorded_at:        Utc::now(),
                            source:             "steam_live".to_string(),
                        }).await;

                        tracing::info!(
                            game = %item.name,
                            old_cut = ?item.discount_percent,
                            new_cut = ?new_discount,
                            "Price changed"
                        );
                        result.prices_changed += 1;
                    }

                    let mut updated = item.clone();
                    updated.current_price    = new_price;
                    updated.original_price   = new_original;
                    updated.discount_percent = new_discount;
                    updated.predicted_regional_low = compute_predicted_regional_low(&updated);
                    updated.is_at_regional_low     = is_at_regional_low(&updated);
                    updated.historical_low         = updated.predicted_regional_low;
                    updated.price_signal = Some(compute_price_signal(&updated));

                    let _ = state.wishlist.upsert(&updated).await;
                }
            }
            Ok(None) => {
                tracing::warn!(game = %item.name, "No app details — possibly removed from Steam");
            }
            Err(e) => {
                tracing::warn!(game = %item.name, error = %e, "Price refresh error");
                result.errors += 1;
            }

        }
    }

    Ok(result)
}

// ─────────────────────────────────────────────
// WISHLIST SYNC
// Compares Steam wishlist against local DB.
// Adds new games, removes de-wishlisted games.
// Enriches new games with ITAD data.
// ─────────────────────────────────────────────

pub async fn sync_wishlist_changes(state: &AppState) -> Result<SyncResult> {
    let settings = state.settings.load().await?;
    let mut result = SyncResult::default();
    
    let steam_id = match &settings.steam_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => {
            tracing::debug!("No Steam ID configured — skipping wishlist sync");
            return Ok(result);
        }
    };
    
    let api_key = match &settings.steam_api_key {
        Some(k) if !k.is_empty() => k.clone(),
        _ => {
            tracing::debug!("No Steam API key configured — skipping wishlist sync");
            return Ok(result);
        }
    };

    let country = settings.country_code.clone();

    tracing::debug!("Checking wishlist for changes");

    // Fetch current wishlist IDs from Steam (just IDs, no appdetails yet)
    // This is cheap — one API call regardless of wishlist size
    state.limits.steam_store.acquire().await;
    let steam_items = match state.steam.get_wishlist_ids(&steam_id, &api_key).await {
        Ok(items) => items,
        Err(e) => {
            tracing::error!(error = %e, "Failed to fetch wishlist IDs from Steam");
            return Ok(result);
        }
    };

    // Get what we currently have locally
    let local_items = state.wishlist.get_all().await?;

    // Build sets for comparison
    // 🦀 RUST LESSON: HashSet for O(1) membership checks
    // Vec::contains() is O(n) — checking 50 items × 50 items = 2500 comparisons
    // HashSet::contains() is O(1) — 50 + 50 = 100 comparisons
    use std::collections::HashSet;

    let steam_ids: HashSet<i64> = steam_items.iter().map(|i| i.appid).collect();
    let local_ids: HashSet<i64> = local_items.iter().map(|i| i.app_id).collect();

    let added_ids: Vec<i64> = steam_ids.difference(&local_ids).copied().collect();

    let removed_ids: Vec<i64> = local_ids.difference(&steam_ids).copied().collect();

    tracing::info!(added = added_ids.len(), removed = removed_ids.len(), "Wishlist diff computed");

    // ── Handle removals ───────────────────────────────────────────
    if !removed_ids.is_empty() {
        match state.wishlist.delete_removed(&steam_ids.into_iter().collect::<Vec<_>>()).await {
            Ok(n) => {
                tracing::info!(count = n, "Removed de-wishlisted games");
                result.games_removed = n as u32;
            }
            Err(e) => {
                tracing::warn!(error = %e, "Failed to remove de-wishlisted games");
                result.errors += 1;
            }
        }
    }

    // ── Handle additions ──────────────────────────────────────────
    if !added_ids.is_empty() {
        tracing::info!(count = added_ids.len(), "Fetching details for new games");

        // Get date_added from the Steam response for each new game
        let date_map: std::collections::HashMap<i64, i64> = steam_items
            .iter()
            .filter(|i| added_ids.contains(&i.appid))
            .map(|i| (i.appid, i.date_added))
            .collect();

        for &app_id in &added_ids {
            // Fetch full details for this new game
            state.limits.steam_store.acquire().await;

            let details = match state.steam.get_app_details(app_id, &country).await {
                Ok(Some(d)) => d,
                Ok(None) => {
                    tracing::warn!(app_id = app_id, "No app details for new game");
                    result.errors += 1;
                    continue;
                }
                Err(e) => {
                    tracing::warn!(app_id = app_id, error = %e, "Failed to fetch new game details");
                    result.errors += 1;
                    continue;
                }
            };

            let date_added     = date_map.get(&app_id).copied().unwrap_or(0);
            let current_price  = details.price_overview.as_ref().map(|p| p.final_price);
            let original_price = details.price_overview.as_ref().map(|p| p.initial);
            let discount       = details.price_overview.as_ref().map(|p| p.discount_percent);
            let header_image   = Some(details.header_image.clone()).filter(|s| !s.is_empty());
            let short_desc     = Some(details.short_description.clone()).filter(|s| !s.is_empty());

            let new_item = crate::models::WishlistItem {
                app_id,
                name:             details.name.clone(),
                date_added:       Some(date_added),
                current_price,
                original_price,
                discount_percent: discount,
                header_image,
                short_description: short_desc,
                // All other fields default to None/empty
                // They'll be populated on next manual enrich
                review_summary:         None,
                reviews_percent:        None,
                reviews_total:          None,
                historical_low:         None,
                buy_recommendation:     None,
                steam_historical_cut:   None,
                steam_historical_date:  None,
                all_time_low_cut:       None,
                all_time_low_shop:      None,
                all_time_low_date:      None,
                predicted_regional_low: None,
                is_at_regional_low:     false,
                price_signal:           None,
                itad_discrepancy:       None,
                steam_review_score:     None,
                steam_review_count:     None,
                review_label:           None,
                opencritic_score:       None,
                metacritic_score:       None,
                release_date:           None,
                tags:                   vec![],
                developers:             vec![],
                itad_id:                None,
                avg_sale_interval_days: None,
                typical_discount_min:   None,
                typical_discount_max:   None,
                last_sale_date:         None,
                predicted_next_sale:    None,
                itad_history_bootstrapped: false,
            };

            match state.wishlist.upsert(&new_item).await {
                Ok(_) => {
                    tracing::info!(game = %details.name, "New wishlist game added");
                    result.games_added += 1;
                    result.games_checked += 1;
                }
                Err(e) => {
                    tracing::warn!(game = %details.name, error = %e, "Failed to save new game");
                    result.errors += 1;
                }
            }
        }

        // Notify user to run Enrich for new games
        if result.games_added > 0 {
            tracing::info!(count = result.games_added, "New games added — run Enrich to fetch price history");
        }
    }
    
    Ok(result)
}

// ─────────────────────────────────────────────
// GAP FILL
// Fetches missing ITAD history for bootstrapped games
// when the app has been offline longer than the gap threshold.
// ─────────────────────────────────────────────

pub async fn fill_history_gaps(state: &AppState) -> Result<()> {
    let settings = state.settings.load().await?;

    let itad_key = match &settings.itad_api_key {
        Some(k) if !k.is_empty() => k.clone(),
        _ => return Ok(()),
    };

    let since_str = settings.last_synced_at
        .map(|ts| ts.format("%Y-%m-%dT%H:%M:%SZ").to_string())
        .unwrap_or_else(|| {
            (Utc::now() - Duration::days(760))
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string()
        });
    
    let wishlist = state.wishlist.get_all().await?;

    for item in wishlist.iter().filter(|i| i.itad_history_bootstrapped) {
        let Some(itad_id) = &item.itad_id else { continue; };

        if let Ok(history) = state.itad.get_price_history(&itad_key, itad_id, &since_str, &state.limits.itad).await {
            for entry in &history {
                if let Ok(ts) = entry.timestamp.parse::<DateTime<Utc>>() {
                    let _ = state.prices.insert(&PricePoint {
                        app_id:           item.app_id,
                        price:            entry.deal.price.amount_int,
                        discount_percent: entry.deal.cut,
                        recorded_at:      ts,
                        source:           "itad_history".to_string(),
                    }).await;
                }
            }
        }
    }

    tracing::debug!("Gap fill complete");
    Ok(())
}

// ─────────────────────────────────────────────
// FULL DAILY SYNC
// Orchestrates all sync steps in order.
// Called by both the command and the background task.
// ─────────────────────────────────────────────

pub async fn run_daily_sync(state: &AppState) -> Result<SyncResult> {
    tracing::info!("Starting daily sync");

    // Step 1: Check for wishlist changes (added/removed games)
    // One cheap API call to get IDs, then details only for new games
    let wishlist_result = sync_wishlist_changes(state).await?;

    // Step 2: Gap fill if needed (before price refresh)
    if needs_gap_fill(state).await {
        tracing::info!("Gap detected — filling history gaps");
        let _ = fill_history_gaps(state).await;
    }

    // Step 3: Light price refresh for all games
    let price_result = refresh_prices(state).await?;

    // Step 4: Discover catalogue price refresh
    // let discover_refreshed = refresh_discover_prices(state).await
    //     .unwrap_or(0);

    // tracing::info!(discover_refreshed, "Discover prices refreshed");

    // Step 5: Save last synced timestamp
    let _ = state.settings
        .set("last_synced_at", &Utc::now().to_rfc3339())
        .await;

    let combined = SyncResult {
        games_checked:  price_result.games_checked,
        prices_changed: price_result.prices_changed,
        games_added:    wishlist_result.games_added,
        games_removed:  wishlist_result.games_removed,
        errors:         price_result.errors + wishlist_result.errors,
    };

    tracing::info!(
        checked = combined.games_checked,
        changed = combined.prices_changed,
        added   = combined.games_added,
        removed = combined.games_removed,
        errors  = combined.errors,
        "Daily sync complete"
    );

    Ok(combined)
}

// / Refresh prices for Discover catalogue games that haven't been
// / checked recently. Only runs for games that have already been
// / fully enriched (have a header_image).
// /
// / Called from the daily sync — updates pricing without re-fetching
// / images or genres (those never change).
// pub async fn refresh_discover_prices(state: &AppState) -> Result<u32> {
//     let settings = state.settings.load().await?;
//     let country  = settings.country_code.clone();
//     let sync_interval_hours = settings.sync_interval_hours.clone();

//     // Only refresh games not checked in the last 24 hours
//     // We don't want to hammer Steam's API on every sync
//     let stale_threshold = Utc::now() - Duration::hours(sync_interval_hours);

//     let stale_games = state.games
//         .get_stale_priced_games(stale_threshold, 50)
//         .await?;

//     if stale_games.is_empty() {
//         tracing::info!("No stale discover games to refresh");
//         return Ok(0);
//     }

//     tracing::info!(count = stale_games.len(), "Refreshing discover game prices");

//     let mut refreshed = 0u32;

//     for mut game in stale_games {
//         state.limits.steam_store.acquire().await;

//         match state.steam.get_app_details(game.app_id, &country).await {
//             Ok(Some(details)) => {
//                 if let Some(price) = &details.price_overview {
//                     game.price_current = Some(price.final_price);
//                     game.price_original = Some(price.initial);
//                 }

//                 game.last_price_check = Some(Utc::now());

//                 // Recompute gem score with fresh price data ????

//                 let _ = state.games.upsert(&game).await;
//                 refreshed += 1;

//                 tracing::info!(
//                     game = %game.name,
//                     price = ?game.price_current,
//                     "Price refreshed"
//                 );
//             }
//             Ok(None) => {
//                 game.last_price_check = Some(Utc::now());
//                 let _ = state.games.upsert(&game).await;
//             }
//             Err(e) => {
//                 tracing::warn!(game = %game.name, error = %e, "Price refresh failed");
//             }
//         }
//     }

//     tracing::info!(refreshed, "Discover price refresh complete");
//     Ok(refreshed)
// }