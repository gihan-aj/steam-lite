use tauri::State;

use crate::AppState;
use crate::models::CrawlState;
use crate::error::AppError;

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