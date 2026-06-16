// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod api;
mod services;
mod db;
mod models;
mod error;

pub use error::{AppError, Result};

// use std::sync::Arc;
use sqlx::SqlitePool;
use tauri::Manager;

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
