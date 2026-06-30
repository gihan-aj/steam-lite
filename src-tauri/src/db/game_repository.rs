use chrono::{DateTime, Utc};
use sqlx::{SqlitePool};
use crate::models::{Game, GameRow};
use crate::error::Result;

// GameRepository holds a REFERENCE to the pool, not the pool itself.
// SqlitePool is already an Arc (reference-counted pointer) internally,
pub struct GameRepository{
    pool: SqlitePool,
}

// All methods on GameRepository live here, separate from the struct definition.
// `&self` means the method borrows the repository (read-only, doesn't consume it)
// This is the equivalent of instance methods in C#.
impl GameRepository {
    pub fn new(pool: SqlitePool) -> Self {
        GameRepository { pool }
    }

    /// Fetches top-rated indie games ordered by review score.
    pub async fn get_top_rated(
        &self,
        min_score: f64,
        limit: i64
    ) -> Result<Vec<Game>>{
        let rows = sqlx::query_as!( 
                GameRow,
                r#"
                SELECT
                    app_id,
                    name,
                    review_score,
                    total_reviews,
                    is_indie as "is_indie: bool",
                    price_current,
                    price_original,
                    platform_windows as "platform_windows: bool",
                    tags,
                    last_updated as "last_updated: chrono::DateTime<chrono::Utc>",
                    gem_score, 
                    owners_lower, 
                    avg_playtime,
                    crawl_source, 
                    header_image, 
                    short_desc, 
                    genres,
                    last_price_check as "last_price_check: chrono::DateTime<chrono::Utc>"
                FROM games
                WHERE review_score >= ?
                AND is_indie = 0
                AND platform_windows = 1
                ORDER BY review_score DESC
                LIMIT ?
                "#,
                min_score,
                limit
            )
            .fetch_all(&self.pool)
            .await?;
        // 🦀 The ? here: if the query returns an Err, we immediately return
        // that error from get_top_rated. If Ok, we unwrap the Vec<Game>.

        // 🦀 RUST LESSON: Iterator chains
        // .into_iter() turns the Vec into an iterator
        // .map(|row| row.into()) converts each GameRow → Game using our From impl
        // .collect() gathers the results back into a Vec<Game>
        // This is Rust's equivalent of LINQ's .Select().ToList()
        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    /// Fetch top hidden gems — games with highest gem_score.
    pub async fn get_hidden_gems(&self, limit: i64, offset: i64,) -> Result<Vec<Game>> {
        let rows = sqlx::query_as!(
            GameRow,
            r#"
            SELECT
                app_id, 
                name, 
                review_score, 
                total_reviews,
                is_indie as "is_indie: bool", 
                price_current, 
                price_original,
                platform_windows as "platform_windows: bool", 
                tags, 
                last_updated as "last_updated: chrono::DateTime<chrono::Utc>",
                gem_score, 
                owners_lower, 
                avg_playtime,
                crawl_source, 
                header_image, 
                short_desc, 
                genres,
                last_price_check as "last_price_check: chrono::DateTime<chrono::Utc>"
            FROM games
            WHERE gem_score IS NOT NULL
                AND gem_score > 0
            ORDER BY gem_score DESC
            LIMIT ? OFFSET ?
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn count_hidden_gems(&self) -> Result<i64> {
        let row = sqlx::query!(
           "SELECT COUNT(*) as count FROM games WHERE gem_score IS NOT NULL AND gem_score > 0"
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(row.count)
    }

    pub async fn get_by_id(&self, app_id: i64) -> Result<Option<Game>> {
        let row = sqlx::query_as!(
            GameRow,
            r#"
            SELECT
                app_id,
                name,
                review_score,
                total_reviews,
                is_indie as "is_indie: bool",
                price_current,
                price_original,
                platform_windows as "platform_windows: bool",
                tags,
                last_updated as "last_updated: chrono::DateTime<chrono::Utc>",
                gem_score, 
                owners_lower, 
                avg_playtime,
                crawl_source, 
                header_image, 
                short_desc, 
                genres,
                last_price_check as "last_price_check: chrono::DateTime<chrono::Utc>"
            FROM games
            WHERE app_id = ?
            "#,
            app_id
        )
        .fetch_optional(&self.pool)
        .await?;

        // 🦀 RUST LESSON: Option::map
        // row is Option<GameRow>
        // .map(|r| r.into()) means: if Some(row), convert it; if None, stay None
        // Result: Option<Game>
        Ok(row.map(|r| r.into()))
    }

    pub async fn upsert(&self, game: &Game) -> Result<()>{
        let is_indie = game.is_indie as i64;
        let platform_windows = game.platform_windows as i64;

        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO games (
                app_id, name, review_score, total_reviews,
                is_indie, price_current, price_original,
                platform_windows, tags, last_updated,
                gem_score, owners_lower, avg_playtime,
                crawl_source, header_image, short_desc, genres,
                last_price_check
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            game.app_id,
            game.name,
            game.review_score,
            game.total_reviews,
            is_indie,
            game.price_current,
            game.price_original,
            platform_windows,
            game.tags,
            game.last_updated,
            game.gem_score,
            game.owners_lower,
            game.avg_playtime,
            game.crawl_source,
            game.header_image,
            game.short_desc,
            game.genres,
            game.last_price_check
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_many(&self, games: &[Game]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for game in games {
            let is_indie = game.is_indie as i64;
            let platform_windows = game.platform_windows as i64;

            sqlx::query!(
                r#"
                INSERT OR REPLACE INTO games (
                    app_id, name, review_score, total_reviews,
                    is_indie, price_current, price_original,
                    platform_windows, tags, last_updated,
                    gem_score, owners_lower, avg_playtime,
                    crawl_source, header_image, short_desc, genres, last_price_check
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                game.app_id,
                game.name,
                game.review_score,
                game.total_reviews,
                is_indie,
                game.price_current,
                game.price_original,
                platform_windows,
                game.tags,
                game.last_updated,
                game.gem_score,
                game.owners_lower,
                game.avg_playtime,
                game.crawl_source,
                game.header_image,
                game.short_desc,
                game.genres,
                game.last_price_check
            )
            .execute(&mut *tx)
            .await?;
        }

        // Commit writes everything to disk atomically.
        // If we return an error before this line, the transaction is
        // automatically rolled back when `tx` is dropped
        tx.commit().await?;

        Ok(())
    }

    /// Count how many games we have cached locally.
    /// Useful for showing sync status in the UI.
    pub async fn count(&self) -> Result<i64> {
        let row = sqlx::query!("SELECT COUNT(*) as count FROM games")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.count)
    }

    /// Delete games not updated within the given number of days.
    /// Called by the sync service to prune stale data.
    pub async fn delete_stale(&self, older_than_days: i64) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM games
            WHERE last_updated < datetime('now', '-' || ? || ' days')
            "#,
            older_than_days
        )
        .execute(&self.pool)
        .await?;

        // rows_affected() tells us how many rows were deleted
        Ok(result.rows_affected())
    }

    // / Get fully-enriched games whose price hasn't been checked recently.
    // / `stale_before` = only return games not checked since this timestamp.
    // / `limit` = max games to return per sync run (avoids long sync times).
//     pub async fn get_stale_priced_games(
//         &self,
//         stale_before: DateTime<Utc>,
//         limit: i64,
//     ) -> Result<Vec<Game>> {
//         let stale_before_str = stale_before.to_rfc3339();
//         let rows = sqlx::query_as!(
//             GameRow,
//             r#"
//             SELECT
//                 app_id, name, review_score, total_reviews,
//                 is_indie, price_current, price_original,
//                 platform_windows, tags, last_updated as "last_updated: chrono::DateTime<chrono::Utc>",
//                 gem_score, owners_lower, avg_playtime,
//                 crawl_source, header_image, short_desc, genres,
//                 last_price_check as "last_price_check: std::option::Option<chrono::DateTime<chrono::Utc>>"
//             FROM games
//             WHERE header_image IS NOT NULL
//                 AND (
//                     last_price_check IS NULL
//                     OR last_price_check < ?
//                 )
//                 AND gem_score IS NOT NULL
//             ORDER BY gem_score DESC
//             LIMIT ?
//             "#,
//             stale_before_str,
//             limit
//         )
//         .fetch_all(&self.pool)
//         .await?;

//         Ok(rows.into_iter().map(|r| r.into()).collect())
//     }
}

