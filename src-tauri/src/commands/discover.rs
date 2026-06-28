use tauri::{AppHandle, State };

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

    // Reset the stop signal to false before starting
    // 🦀 RUST LESSON: watch::Sender::send
    // Sends a new value to all receivers.
    // We set false = "don't stop" before launching.
    let _ = state.crawl_stop_tx.send(false);

    // Subscribe a new receiver for this crawl run
    let stop_rx = state.crawl_stop_tx.subscribe();

    let app_clone = app.clone();

    tracing::info!("Starting catalogue crawl task");

    tauri::async_runtime::spawn(async move {
        if let Err(e) = catalogue_crawler::run_crawl_with_handle(
            app_clone.clone(),
            stop_rx,
        ).await {
            tracing::error!(error = %e, "Crawl task failed");
        }
    });

    Ok(())
}

/// Stop the currently running crawl gracefully.
/// The crawl saves its current page and exits cleanly.
#[tauri::command]
pub async fn stop_crawl(
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    tracing::info!("Stop signal sent to crawl");
    let _ = state.crawl_stop_tx.send(true);
    Ok(())
}

/// Get the top hidden gems from our catalogue.
#[tauri::command]
pub async fn get_hidden_gems(
    state: State<'_, AppState>,
    limit: i64,
) -> Result<Vec<crate::models::Game>, AppError> {
    state.games.get_hidden_gems(limit).await
}