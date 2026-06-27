// Reads and writes crawl progress to the crawl_state table.
// The table is just key-value pairs — simple and robust.
// If the app crashes mid-crawl, the last committed page number
// is preserved and we resume from there.

use sqlx::SqlitePool;

use crate::{Result, models::{CrawlState, CrawlStatus}};

pub struct CrawlRepository {
    pool: SqlitePool
}

impl CrawlRepository {
    pub fn new(pool: SqlitePool) -> Self {
        CrawlRepository { pool }
    }

    /// Load full crawl state from DB
    pub async fn load(&self) -> Result<CrawlState> {
        let rows = sqlx::query!(
            "SELECT key, value FROM crawl_state"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut state = CrawlState::default();
        state.total_pages = 50; // default if not stored

        for row in rows {
            match row.key.as_deref() {
                Some("current_page")    => state.current_page    = row.value.parse().unwrap_or(0),
                Some("total_pages")     => state.total_pages     = row.value.parse().unwrap_or(50),
                Some("games_indexed")   => state.games_indexed   = row.value.parse().unwrap_or(0),
                Some("games_qualified") => state.games_qualified = row.value.parse().unwrap_or(0),
                Some("last_page_at")    => {
                    state.last_page_at = if row.value.is_empty() {
                        None
                    } else {
                        Some(row.value.to_string())
                    }
                }
                Some("status") => {
                    state.status = match row.value.as_str() {
                        "running"  => CrawlStatus::Running,
                        "paused"   => CrawlStatus::Paused,
                        "complete" => CrawlStatus::Complete,
                        _          => CrawlStatus::Idle,
                    }
                }
                _ => {}
            }
        }

        Ok(state)
    }

    /// Save a single field — called after each page completes.
    /// We save incrementally so crashes only lose the current page.
    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO crawl_state (key, value) VALUES (?, ?)",
            key, value
        )
        .execute(&self.pool)
        .await?;
       Ok(())
    }

    /// Advance to the next page and record when we did so.
    /// Called after each page is successfully processed.
    pub async fn advance_page(&self, page: u32, indexed: u32, qualified: u32) -> Result<()> {
        use chrono::Utc;

        let mut tx = self.pool.begin().await?;

        let next_page = (page + 1).to_string();
        let indexed_str = indexed.to_string();
        let qual_str    = qualified.to_string();
        let now         = Utc::now().to_rfc3339();

        sqlx::query!(
            "INSERT OR REPLACE INTO crawl_state (key, value) VALUES ('current_page', ?)",
            next_page
        ).execute(&mut *tx).await?;

        sqlx::query!(
            "INSERT OR REPLACE INTO crawl_state (key, value) VALUES ('games_indexed', ?)",
            indexed_str
        ).execute(&mut *tx).await?;

        sqlx::query!(
            "INSERT OR REPLACE INTO crawl_state (key, value) VALUES ('games_qualified', ?)",
            qual_str
        ).execute(&mut *tx).await?;

        sqlx::query!(
            "INSERT OR REPLACE INTO crawl_state (key, value) VALUES ('last_page_at', ?)",
            now
        ).execute(&mut *tx).await?;

        tx.commit().await?;

        Ok(())
    }

    /// Mark the crawl as running — called when starting/resuming.
    pub async fn set_status(&self, status: CrawlStatus) -> Result<()> {
        self.set("status", &status.to_string()).await
    }

    /// Reset crawl to start from page 0.
    pub async fn reset(&self) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for (key, value) in &[
            ("current_page",    "0"),
            ("games_indexed",   "0"),
            ("games_qualified", "0"),
            ("last_page_at",    ""),
            ("status",          "idle"),
        ] {
            sqlx::query!(
                "INSERT OR REPLACE INTO crawl_state (key, value) VALUES (?, ?)",
                key, value
            ).execute(&mut *tx).await?;
        }

        tx.commit().await?;
        Ok(())
    }
}