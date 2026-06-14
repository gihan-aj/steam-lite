use sqlx::SqlitePool;
use crate::models::{WishlistItem, WishlistRow};
use crate::error::Result;

pub struct WishlistRepository {
    pool: SqlitePool,
}

impl WishlistRepository {
    pub fn new(pool: SqlitePool) -> Self {
        WishlistRepository { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<WishlistItem>> {
        let rows = sqlx::query_as!(
            WishlistRow,
            r#"
            SELECT
                app_id,
                name,
                review_summary,
                reviews_percent,
                reviews_total,
                date_added,
                current_price,
                historical_low,
                discount_percent
            FROM wishlist
            ORDER BY name ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // buy_recommendation starts as None here —
        // the service layer fills it in after fetching price history
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn upsert(&self, item: &WishlistItem) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO wishlist (
                app_id, name, review_summary, reviews_percent,
                reviews_total, date_added, current_price,
                historical_low, discount_percent, last_checked
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            "#,
            item.app_id,
            item.name,
            item.review_summary,
            item.reviews_percent,
            item.reviews_total,
            item.date_added,
            item.current_price,
            item.historical_low,
            item.discount_percent
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, app_id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM wishlist WHERE app_id = ?", app_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn count(&self) -> Result<i64> {
        let row = sqlx::query!("SELECT COUNT(*) as count FROM wishlist")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.count)
    }
}