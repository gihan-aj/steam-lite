use chrono::Utc;
use tauri::{AppHandle, Emitter, State };

use crate::AppState;
use crate::models::{CrawlState, CrawlStatus};
use crate::error::AppError;
use crate::services::catalogue_crawler;

/// Get the current crawl progress.
/// Called by the Discover page to show progress to the user.
#[tauri::command]
pub async fn get_crawl_state(
    state: State<'_, AppState>,
) -> Result<CrawlState, AppError> {
    state.crawl.load().await
}

/// Reset the crawl — start from page 0 again.
/// Useful if SteamSpy data is very stale or something went wrong.
#[tauri::command]
pub async fn reset_crawl(
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    {
        let mut task_guard = state.crawl_task.lock().await;
        if let Some(old_handle) = task_guard.take() {
            tracing::info!("Aborting crawl task for reset");
            old_handle.abort();
        }
    }

    let _ = state.crawl_stop_tx.send_replace(true);

    state.crawl.reset().await
}

/// Start (or resume) the catalogue crawl.
/// Spawns a background Tokio task — returns immediately to the UI.
#[tauri::command]
pub async fn start_crawl(
    state:  State<'_, AppState>,
    app:    AppHandle,
) -> Result<(), AppError> {
    let crawl = state.crawl.load().await?;

    if crawl.status == CrawlStatus::Running {
        tracing::info!("Crawl already running");
        return Ok(());
    }

    if crawl.status == CrawlStatus::Complete {
        tracing::info!("Crawl already complete");
        return Ok(());
    }

    {
        let mut task_guard = state.crawl_task.lock().await;
        if let Some(old_handle) = task_guard.take() {
            tracing::info!("Aborting previous crawl task before resume");
            old_handle.abort();
        }
    }
    
    // Reset the stop signal to false before starting
    // 🦀 RUST LESSON: watch::Sender::send
    // Sends a new value to all receivers.
    // We set false = "don't stop" before launching.
    let _ = state.crawl_stop_tx.send_replace(false);
    state.crawl.set_status(CrawlStatus::Running).await?;

    // Subscribe a new receiver for this crawl run
    let stop_rx = state.crawl_stop_tx.subscribe();
    let app_clone = app.clone();

    tracing::info!("Starting catalogue crawl task");

    let handle = tauri::async_runtime::spawn(async move {
        if let Err(e) = catalogue_crawler::run_crawl_with_handle(
            app_clone.clone(),
            stop_rx,
        ).await {
            tracing::error!(error = %e, "Crawl task failed");
        }
    });

    *state.crawl_task.lock().await = Some(handle);

    Ok(())
}

/// Stop the currently running crawl gracefully.
/// The crawl saves its current page and exits cleanly.
#[tauri::command]
pub async fn stop_crawl(
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    tracing::info!("Stop signal sent to crawl");
    let _ = state.crawl_stop_tx.send_replace(true);
    Ok(())
}

/// Get the top hidden gems from our catalogue.
#[tauri::command]
pub async fn get_hidden_gems(
    state: State<'_, AppState>,
    limit: i64,
    offset: i64,
) -> Result<Vec<crate::models::Game>, AppError> {
    state.games.get_hidden_gems(limit, offset).await
}

#[tauri::command]
pub async fn count_hidden_gems(
    state: State<'_, AppState>,
) -> Result<i64, AppError> {
    state.games.count_hidden_gems().await
}

/// Fetch Steam appdetails for games that don't have images yet.
/// Called by React when the Discover page loads.
/// Returns how many games were enriched.
#[tauri::command]
pub async fn enrich_discover_games(
    state: State<'_, AppState>,
    app: AppHandle,
    app_ids: Vec<i64>,
) -> Result<usize, AppError>{
    let settings = state.settings.load().await?;
    let country = settings.country_code.clone();
    let sync_interval_hours = settings.sync_interval_hours.clone();
    let mut enriched = 0;

    let stale_threshold = Utc::now() - chrono::Duration::hours(sync_interval_hours);

    for app_id in &app_ids {
        let existing = state.games.get_by_id(*app_id).await.ok().flatten();

        if let Some(game) = &existing {
            let has_image = game.header_image.is_some();
            let is_fresh = game.last_price_check
                .map(|t| t > stale_threshold)
                .unwrap_or(false);

            if has_image && is_fresh {
                continue;
            }
        }

        state.limits.steam_store.acquire().await;

        if let Ok(Some(details)) = state.steam.get_app_details(*app_id, &country).await {
            if let Ok(Some(mut game)) = state.games.get_by_id(*app_id).await {
                game.header_image = Some(details.header_image.clone())
                    .filter(|s| !s.is_empty())
                    .or(game.header_image);   // keep old image if Steam returns empty

                game.short_desc = Some(details.short_description.clone())
                    .filter(|s| !s.is_empty())
                    .or(game.short_desc);

                let genres: Vec<String> = details.genres
                    .iter()
                    .map(|g| g.description.clone())
                    .collect();

                game.genres = serde_json::to_string(&genres).ok();
                game.is_indie = details.is_indie();

                game.last_price_check = Some(Utc::now());

                // Update price with regional data
                if let Some(price) = &details.price_overview {
                    game.price_current  = Some(price.final_price);
                    game.price_original = Some(price.initial);
                }

                let _ = state.games.upsert(&game).await;
                enriched += 1;

                let _ = app.emit("game_enriched", serde_json::json!({
                    "app_id": app_id,
                }));
            }
        }
    }
    tracing::info!(enriched, "Discover enrichment/refresh complete");
    Ok(enriched)
}