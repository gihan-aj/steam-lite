use crate::error::Result;
use crate::models::UserSettings;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

pub struct SettingsRepository {
    pool: SqlitePool,
}

impl SettingsRepository {
    pub fn new(pool: SqlitePool) -> Self {
        SettingsRepository { pool }
    }

    pub async fn load(&self) -> Result<UserSettings> {
        // Helper closure to fetch one setting value by key
        // 🦀 RUST LESSON: closures
        // Closures in Rust are anonymous functions: |args| body
        // Similar to lambdas in C#. They can capture variables from
        // the surrounding scope (here: `self`).
        let get = |key: &'static str| async move {
            sqlx::query!("SELECT value FROM user_settings WHERE key = ?", key)
                .fetch_optional(&self.pool)
                .await
        };

        // Fetch each setting, falling back to defaults if missing
        let min_score = get("min_review_score")
            .await?
            .and_then(|r| r.value.parse::<f64>().ok())
            .unwrap_or(90.0);

        let min_discount = get("min_discount_percent")
            .await?
            .and_then(|r| r.value.parse::<i64>().ok())
            .unwrap_or(50);

        let sync_interval = get("sync_interval_hours")
            .await?
            .and_then(|r| r.value.parse::<i64>().ok())
            .unwrap_or(24);

        let last_synced_at = get("last_synced_at")
            .await?
            .map(|r| r.value)
            .filter(|v| !v.is_empty())
            .and_then(|r| r.parse::<DateTime<Utc>>().ok());

        let alert_threshold = get("alert_threshold_percent")
            .await?
            .and_then(|r| r.value.parse::<i64>().ok())
            .unwrap_or(50);

        let steam_api_key = get("steam_api_key")
            .await?
            .map(|r| r.value)
            .filter(|v| !v.is_empty());

        let steam_id = get("steam_id").await?.map(|r| r.value);

        let itad_api_key = get("itad_api_key")
            .await?
            .map(|r| r.value)
            .filter(|v| !v.is_empty());

        let country_code = get("country_code")
            .await?
            .map(|r| r.value)
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| "lk".to_string());

        Ok(UserSettings {
            steam_id,
            steam_api_key,
            itad_api_key,
            country_code,
            min_review_score: min_score,
            min_discount_percent: min_discount,
            sync_interval_hours: sync_interval,
            last_synced_at: last_synced_at,
            alert_threshold_percent: alert_threshold,
        })
    }

    /// Save a single setting by key.
    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO user_settings (key, value) VALUES (?, ?)",
            key,
            value
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Convenience method to save the whole UserSettings struct at once.
    pub async fn save(&self, settings: &UserSettings) -> Result<()> {
        // 🦀 RUST LESSON: if let with Option
        // We only save steam_id if the user has actually set one.
        // `if let Some(id) = &settings.steam_id` means:
        // "if steam_id is Some(value), bind that value to `id` and run the block"
        // If it's None, the block is skipped entirely.
        if let Some(id) = &settings.steam_id {
            self.set("steam_id", id).await?;
        }
        if let Some(key) = &settings.steam_api_key {
            self.set("steam_api_key", key).await?;
        }
        if let Some(key) = &settings.itad_api_key {
            self.set("itad_api_key", key).await?;
        }
        if let Some(ts) = settings.last_synced_at {
            self.set("last_synced_at", &ts.to_rfc3339()).await?;
        }

        self.set("country_code", &settings.country_code).await?;
        self.set("min_review_score", &settings.min_review_score.to_string())
            .await?;
        self.set(
            "min_discount_percent",
            &settings.min_discount_percent.to_string(),
        )
        .await?;
        self.set(
            "sync_interval_hours",
            &settings.sync_interval_hours.to_string(),
        )
        .await?;
        self.set(
            "alert_threshold_percent",
            &settings.alert_threshold_percent.to_string(),
        )
        .await?;

        Ok(())
    }
}
