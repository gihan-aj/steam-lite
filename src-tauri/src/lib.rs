// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod api;
mod services;
mod db;
mod models;
mod error;

use commands::*; // Bring all public command functions into scope
pub use error::{AppError, Result};
use tauri::Manager;

use std::sync::Arc;
use sqlx::SqlitePool;

pub struct AppState{
    pub db: SqlitePool,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
            app.manage(AppState { db: db_pool });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
