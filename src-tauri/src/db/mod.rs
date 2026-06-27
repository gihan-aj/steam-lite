pub mod game_repository;
pub mod price_repository;
pub mod wishlist_repository;
pub mod settings_repository;
pub mod crawl_repository;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use crate::error::Result;

/// Initialises the database connection pool and runs any pending migrations.
///
/// This is called once at app startup (in lib.rs).
/// After this returns, the database file exists, all tables are created,
/// and the app is ready to read/write data.
///
/// `db_path` is the path to the .db file, e.g. "steam-lite.db"
pub async fn init_db(db_path: &str) -> Result<SqlitePool> {
    let connection_string = format!("sqlite:{}?mode=rwc", db_path);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&connection_string)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to open database at {}: {}", db_path, e))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;

    Ok(pool)
}