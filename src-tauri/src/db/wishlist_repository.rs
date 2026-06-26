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
                app_id, name, review_summary,
                reviews_percent, reviews_total, date_added,
                current_price, original_price, historical_low,
                discount_percent, header_image, short_description,
                steam_historical_cut, steam_historical_date,
                all_time_low_cut, all_time_low_shop, all_time_low_date,
                predicted_regional_low, is_at_regional_low,
                steam_review_score, steam_review_count, review_label,
                opencritic_score, metacritic_score,
                release_date, tags, developers, itad_id,
                avg_sale_interval_days, typical_discount_min,
                typical_discount_max, last_sale_date, predicted_next_sale, itad_history_bootstrapped
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

    /// Returns all app_ids currently in the local wishlist.
    pub async fn get_all_app_ids(&self) -> Result<Vec<i64>> {
        let rows = sqlx::query!(
            "SELECT app_id FROM wishlist"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.app_id).collect())
    }

    /// Returns a WishlistItem by app_id. If not found, returns None.
    pub async fn get_by_id(&self, app_id: i64) -> Result<Option<WishlistItem>> {
        let row = sqlx::query_as!(
            WishlistRow,
            r#"
            SELECT
                app_id, name, review_summary,
                reviews_percent, reviews_total, date_added,
                current_price, original_price, historical_low,
                discount_percent, header_image, short_description,
                steam_historical_cut, steam_historical_date,
                all_time_low_cut, all_time_low_shop, all_time_low_date,
                predicted_regional_low, is_at_regional_low,
                steam_review_score, steam_review_count, review_label,
                opencritic_score, metacritic_score,
                release_date, tags, developers, itad_id,
                avg_sale_interval_days, typical_discount_min,
                typical_discount_max, last_sale_date, predicted_next_sale, itad_history_bootstrapped
            FROM wishlist
            WHERE app_id = ?
            "#,
            app_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    pub async fn upsert(&self, item: &WishlistItem) -> Result<()> {
        let is_at_regional_low = item.is_at_regional_low as i64;
        let tags = serde_json::to_string(&item.tags).ok()
                .and_then(|s| if s == "null" { None } else { Some(s) });
        let devs = serde_json::to_string(&item.developers).ok()
                .and_then(|s| if s == "null" { None } else { Some(s) });


        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO wishlist (
                app_id, name, review_summary, reviews_percent,
                reviews_total, date_added, current_price, original_price,
                historical_low, discount_percent, header_image, short_description,
                steam_historical_cut, steam_historical_date,
                all_time_low_cut, all_time_low_shop, all_time_low_date,
                predicted_regional_low, is_at_regional_low,
                steam_review_score, steam_review_count, review_label,
                opencritic_score, metacritic_score,
                release_date, tags, developers, itad_id,
                avg_sale_interval_days, typical_discount_min,
                typical_discount_max, last_sale_date, predicted_next_sale, itad_history_bootstrapped,
                last_checked
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                CURRENT_TIMESTAMP
            )
            "#,
            item.app_id,
            item.name,
            item.review_summary,
            item.reviews_percent,
            item.reviews_total,
            item.date_added,
            item.current_price,
            item.original_price,
            item.historical_low,
            item.discount_percent,
            item.header_image,
            item.short_description,
            item.steam_historical_cut,
            item.steam_historical_date,
            item.all_time_low_cut,
            item.all_time_low_shop,
            item.all_time_low_date,
            item.predicted_regional_low,
            is_at_regional_low,
            item.steam_review_score,
            item.steam_review_count,
            item.review_label,
            item.opencritic_score,
            item.metacritic_score,
            item.release_date,
            tags,
            devs,
            item.itad_id,
            item.avg_sale_interval_days,
            item.typical_discount_min,
            item.typical_discount_max,
            item.last_sale_date,
            item.predicted_next_sale,
            item.itad_history_bootstrapped
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

    /// Delete all wishlist entries whose app_id is NOT in the provided list.
    /// Called after a successful Steam sync to remove de-wishlisted games.
    /// Does NOT touch price_history — historical data is preserved.
    pub async fn delete_removed(&self, current_app_ids: &[i64]) -> Result<u64> {
        if current_app_ids.is_empty() {
            // Safety guard: if Steam returned nothing (API error?),
            // don't wipe the entire local wishlist.
            // An empty list likely means something went wrong, not
            // that the user removed every game.
            tracing::warn!("delete_removed called with empty list — skipping to avoid accidental wipe");
            return Ok(0);
        }

        // 🦀 RUST LESSON: dynamic IN clauses with sqlx
        // sqlx's query! macro doesn't support dynamic IN lists directly
        // because the number of bind parameters isn't known at compile time.
        //
        // QueryBuilder is the idiomatic sqlx solution — it builds the SQL
        // string internally and binds values safely (no string interpolation
        // of user data, so no injection risk).
        let mut builder = sqlx::QueryBuilder::new(
            "DELETE FROM wishlist WHERE app_id NOT IN ("
        );

        // push_values iterates the slice and emits one bind parameter per item.
        // sqlx handles the comma-separated "?, ?, ?" internally.
        let mut separated = builder.separated(", ");
        for id in current_app_ids {
            separated.push_bind(*id);
        }
        builder.push(")");

        let result = builder.build().execute(&self.pool).await?;
        Ok(result.rows_affected())
    }
}