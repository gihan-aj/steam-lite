// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod api;
mod commands;
mod db;
mod error;
mod models;
mod services;

use chrono::Utc;
pub use error::{AppError, Result};

// use std::sync::Arc;
use sqlx::SqlitePool;
use tauri::{Emitter, Manager};

use db::{
    crawl_repository::CrawlRepository, game_repository::GameRepository,
    price_repository::PriceRepository, settings_repository::SettingsRepository,
    wishlist_repository::WishlistRepository,
};
use tokio::sync::watch;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::api::itad::ItadClient;
use crate::api::steam::SteamClient;
use crate::api::steamspy::SteamSpyClient;
use crate::models::CrawlStatus;
use crate::services::catalogue_crawler;
use api::rate_limiter::ApiRateLimiters;

pub struct AppState {
    pub db: SqlitePool,
    pub games: GameRepository,
    pub prices: PriceRepository,
    pub wishlist: WishlistRepository,
    pub settings: SettingsRepository,
    pub crawl: CrawlRepository,
    pub steamspy: SteamSpyClient,
    pub steam: SteamClient,
    pub itad: ItadClient,
    pub limits: ApiRateLimiters,
    pub crawl_stop_tx: watch::Sender<bool>,
    pub crawl_task: tokio::sync::Mutex<Option<tauri::async_runtime::JoinHandle<()>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logging();

    tracing::info!("Steam Lite starting up");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let db_pool = tauri::async_runtime::block_on(async {
                // C:\Users\{name}\AppData\Roaming\{app-name}\
                let app_data_dir = app
                    .path()
                    .app_data_dir()
                    .expect("Could not resolve app data directory");
                std::fs::create_dir_all(&app_data_dir)
                    .expect("Could not create app data directory");
                let db_path = app_data_dir
                    .join("steam-lite.db")
                    .to_string_lossy()
                    .to_string();

                db::init_db(&db_path)
                    .await
                    .expect("Failed to initialise database")
            });

            let (crawl_stop_tx, _crawl_stop_rx) = watch::channel(false);

            // Register the pool in Tauri's state container
            let state = AppState {
                games: GameRepository::new(db_pool.clone()),
                prices: PriceRepository::new(db_pool.clone()),
                wishlist: WishlistRepository::new(db_pool.clone()),
                settings: SettingsRepository::new(db_pool.clone()),
                crawl: CrawlRepository::new(db_pool.clone()),
                steamspy: SteamSpyClient::new(),
                steam: SteamClient::new(),
                itad: ItadClient::new(),
                limits: ApiRateLimiters::new(),
                crawl_stop_tx,
                crawl_task: tokio::sync::Mutex::new(None),
                db: db_pool,
            };
            app.manage(state);

            use crate::services::sync_service;

            let bg_handle = app.handle().clone();

            // 🦀 RUST LESSON: tokio::spawn
            // Spawns an async task that runs concurrently with everything else.
            // The task runs for the lifetime of the app.
            // `move` captures app_handle by value (moves ownership into the closure).
            tauri::async_runtime::spawn(async move {
                // Check every 30 minutes
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30 * 60));
                interval.tick().await; // skip immediate first tick

                loop {
                    interval.tick().await;

                    let state = bg_handle.state::<AppState>();

                    if !sync_service::is_sync_due(&state).await {
                        tracing::trace!("Background sync: not due yet");
                        continue;
                    }

                    tracing::info!("Background sync: running daily sync");

                    match sync_service::run_daily_sync(&state).await {
                        Ok(result) => {
                            let _ = bg_handle.emit(
                                "prices_refreshed",
                                serde_json::json!({
                                    "updated":  result.games_checked,
                                    "changed":  result.prices_changed,
                                    "added":    result.games_added,
                                    "removed":  result.games_removed,
                                    "source":   "background",
                                }),
                            );
                        }
                        Err(e) => {
                            tracing::error!(error = %e, "Background sync failed");
                        }
                    }
                }
            });

            // Trigger a sync check immediately on app open
            // This handles the "app was closed for 2 days" case
            let startup_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // Small delay — let the window finish loading first
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

                let state = startup_handle.state::<AppState>();
                let settings = match state.settings.load().await {
                    Ok(s) => s,
                    Err(_) => return,
                };

                let is_due = settings
                    .last_synced_at
                    .map(|last| (Utc::now() - last).num_hours() >= settings.sync_interval_hours)
                    .unwrap_or(true);

                if is_due {
                    tracing::info!("Startup: sync is due, triggering");
                    // Emit to React — it will call the refresh_prices command
                    let _ = startup_handle.emit("sync_due", ());
                } else {
                    tracing::debug!("Startup: sync not due yet");
                }
            });

            let crawl_resume_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

                let state = crawl_resume_handle.state::<AppState>();

                let crawl = match state.crawl.load().await {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!(error = %e, "Startup: failed to load crawl state");
                        return;
                    }
                };

                if crawl.status != CrawlStatus::Running {
                    tracing::debug!(status = ?crawl.status, "Startup: crawl not auto-resuming");
                    return;
                }

                tracing::info!(
                    page = crawl.current_page,
                    "Startup: crawl was interuppted - auto-resuming"
                );

                state.crawl_stop_tx.send_replace(false);
                let stop_rx = state.crawl_stop_tx.subscribe();

                let app_clone = crawl_resume_handle.clone();

                let handle = tauri::async_runtime::spawn(async move {
                    if let Err(e) =
                        catalogue_crawler::run_crawl_with_handle(app_clone, stop_rx).await
                    {
                        tracing::error!(error = %e, "Startup: auto-resume crawl failed");
                    }
                });

                *state.crawl_task.lock().await = Some(handle);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::games::get_recommended_games,
            commands::games::sync_games,
            commands::games::get_game_details,
            commands::settings::get_settings,
            commands::settings::set_setting,
            commands::settings::save_settings,
            commands::settings::get_log_path,
            commands::settings::open_log_folder,
            commands::wishlist::fetch_wishlist,
            commands::wishlist::get_wishlist,
            commands::wishlist::remove_from_wishlist,
            commands::wishlist::enrich_wishlist_prices,
            commands::wishlist::get_game_price_history,
            commands::wishlist::refresh_prices,
            commands::discover::get_crawl_state,
            commands::discover::reset_crawl,
            commands::discover::start_crawl,
            commands::discover::stop_crawl,
            commands::discover::get_hidden_gems,
            commands::discover::count_hidden_gems,
            commands::discover::enrich_discover_games,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_logging() {
    // On Windows: C:\Users\{user}\AppData\Roaming\steam-lite\logs\
    // On macOS:   ~/Library/Application Support/steam-lite/logs/
    // On Linux:   ~/.local/share/steam-lite/logs/
    let log_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("steam-lite")
        .join("logs");

    std::fs::create_dir_all(&log_dir)
        .expect("Failed to create log directory");

    let file_appender = tracing_appender::rolling::daily(&log_dir, "steam-lite.log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    // Leak it intentionally — it lives for the whole process lifetime
    std::mem::forget(_guard);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("steam_lite=info,warn"));

    // File layer — writes structured logs to disk
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)        // no colour codes in log files
        .with_target(true)       // include module path
        .with_thread_ids(false)
        .compact();

    // Console layer — only in debug builds (dev mode)
    #[cfg(debug_assertions)]
    let console_layer = Some(
        tracing_subscriber::fmt::layer()
            .with_target(false)
            .compact()
    );
    #[cfg(not(debug_assertions))]
    let console_layer: Option<tracing_subscriber::fmt::Layer<_>> = None;

    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer)
        .init();
}