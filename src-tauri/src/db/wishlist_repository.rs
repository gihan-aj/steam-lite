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
                typical_discount_max, last_sale_date, predicted_next_sale
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
                typical_discount_max, last_sale_date, predicted_next_sale,
                last_checked
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
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
            item.predicted_next_sale
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