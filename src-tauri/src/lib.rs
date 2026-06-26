// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod api;
mod services;
mod db;
mod models;
mod error;

use chrono::Utc;
pub use error::{AppError, Result};

// use std::sync::Arc;
use sqlx::SqlitePool;
use tauri::{Emitter, Manager};

use db::{
    game_repository::GameRepository,
    price_repository::PriceRepository,
    wishlist_repository::WishlistRepository,
    settings_repository::SettingsRepository,
};

use crate::api::itad::ItadClient;
use crate::api::steamspy::SteamSpyClient;
use crate::api::steam::SteamClient;
use api::rate_limiter::ApiRateLimiters;

pub struct AppState{
    pub db: SqlitePool,
    pub games: GameRepository,
    pub prices: PriceRepository,
    pub wishlist: WishlistRepository,
    pub settings: SettingsRepository,
    pub steamspy: SteamSpyClient,
    pub steam: SteamClient,
    pub itad: ItadClient,
    pub limits:   ApiRateLimiters,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 🦀 RUST LESSON: tracing subscriber
    // tracing is Rust's structured logging framework — like Serilog in .NET.
    // The subscriber is what actually processes log records.
    // EnvFilter reads RUST_LOG env var: RUST_LOG=steam_lite=debug cargo run
    // Default here: show info and above from our crate, warn from dependencies.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "steam_lite=info,warn".into())
        )
        .with_target(false) //   Don't show module path
        .with_thread_ids(false)
        .compact()
        .init();

    tracing::info!("Steam Lite starting up");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let db_pool = tauri::async_runtime::block_on(async {
                // Gets the app's data directory — on Windows this is:
                // C:\Users\{name}\AppData\Roaming\{app-name}\
                // This is the correct place to store user data on all platforms.
                let app_data_dir = app.path().app_data_dir()
                    .expect("Could not resolve app data directory");
                std::fs::create_dir_all(&app_data_dir)
                    .expect("Could not create app data directory");
                let db_path = app_data_dir
                    .join("steam-lite.db")
                    .to_string_lossy()
                    .to_string();

                db::init_db(&db_path).await
                    .expect("Failed to initialise database")
            });

            // Register the pool in Tauri's state container
            let state = AppState {
                games:    GameRepository::new(db_pool.clone()),
                prices:   PriceRepository::new(db_pool.clone()),
                wishlist: WishlistRepository::new(db_pool.clone()),
                settings: SettingsRepository::new(db_pool.clone()),
                steamspy: SteamSpyClient::new(),
                steam:    SteamClient::new(), 
                itad:     ItadClient::new(),
                limits:   ApiRateLimiters::new(),
                db:       db_pool,
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
                let mut interval = tokio::time::interval(
                    tokio::time::Duration::from_secs(30 * 60)
                );
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
                            let _ = bg_handle.emit("prices_refreshed", serde_json::json!({
                                "updated":  result.games_checked,
                                "changed":  result.prices_changed,
                                "added":    result.games_added,
                                "removed":  result.games_removed,
                                "source":   "background",
                            }));
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

                let is_due = settings.last_synced_at
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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::games::get_recommended_games,
            commands::games::sync_games,
            commands::games::get_game_details,
            commands::settings::get_settings,
            commands::settings::set_setting,
            commands::settings::save_settings,
            commands::wishlist::fetch_wishlist,
            commands::wishlist::get_wishlist, 
            commands::wishlist::remove_from_wishlist,
            commands::wishlist::enrich_wishlist_prices,
            commands::wishlist::get_game_price_history,
            commands::wishlist::refresh_prices,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
