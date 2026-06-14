use sqlx::SqlitePool;
use crate::models::{PricePoint, PriceRow};
use crate::error::Result;

pub struct PriceRepository {
    pool: SqlitePool,
}

impl PriceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        PriceRepository { pool }
    }

    /// Append a new price record. Never updates — price history is immutable.
    pub async fn insert(&self, point: &PricePoint) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO price_history (app_id, price, discount_percent, recorded_at, source)
            VALUES (?, ?, ?, ?, ?)
            "#,
            point.app_id,
            point.price,
            point.discount_percent,
            point.recorded_at,
            point.source
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get full price history for a game, newest first.
    pub async fn get_history(&self, app_id: i64) -> Result<Vec<PricePoint>> {
        let rows = sqlx::query_as!(
            PriceRow,
            r#"
            SELECT
                app_id,
                price,
                discount_percent,
                recorded_at as "recorded_at: chrono::DateTime<chrono::Utc>",
                source
            FROM price_history
            WHERE app_id = ?
            ORDER BY recorded_at DESC
            "#,
            app_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    /// Get the historical lowest price ever recorded for a game.
    pub async fn get_historical_low(&self, app_id: i64) -> Result<Option<i64>> {
        let row = sqlx::query!(
            "SELECT MIN(price) as low FROM price_history WHERE app_id = ?",
            app_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.and_then(|r| r.low))
        // 🦀 RUST LESSON: .and_then() on Option
        // row is Option<Row>. .and_then() says:
        // "if Some(row), apply this closure and return its result"
        // "if None, return None"
        // It's like chaining .map() but flattens nested Options.
        // Equivalent to: if let Some(r) = row { r.low } else { None }
    }
}