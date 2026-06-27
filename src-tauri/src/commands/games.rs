use tauri::State;
use crate::AppState;
use crate::models::{Game, RecommendedGame};
use crate::error::AppError;

/// Fetch recommended indie games from local cache.
/// If cache is empty, fetches from SteamSpy first.
#[tauri::command]
pub async fn get_recommended_games(
    state: State<'_, AppState>, 
    min_score: f64,
    limit: i64
) -> Result<Vec<RecommendedGame>, AppError> {
    let count = state.games.count().await?;
    tracing::debug!(count, min_score, "get_recommended_games called");

    if count == 0 {
        // Cache is empty — fetch from SteamSpy and store
        // This happens on first run or after a database reset
        tracing::info!("Game cache empty — triggering sync");
        sync_games_internal(&state).await?;
    }

    // Fetch from local SQLite (fast, no network needed)
    let games = state.games.get_top_rated(min_score, limit).await?;
    tracing::debug!(count = games.len(), "get_top_rated returned games");

    // Convert Game → RecommendedGame by scoring each one
    // 🦀 RUST LESSON: .iter().map().collect() — the Rust LINQ equivalent
    // .iter()    → iterate by reference (don't consume the Vec)
    // .map()     → transform each element
    // .collect() → gather into a new Vec
    let recommended : Vec<RecommendedGame> = games
        .iter()
        .map(|game| score_game(game))
        .collect();

    tracing::debug!(count = recommended.len(), "Returning recommended games");
    Ok(recommended)
}

/// Trigger a manual sync from SteamSpy — fetches fresh data and stores it.
///
/// Called from React like:
///   invoke('sync_games')
///
/// Returns the number of games synced.
#[tauri::command]
pub async fn sync_games(
    state: State<'_, AppState>,
) -> Result<usize, AppError> {
    sync_games_internal(&state).await
}

/// Get a single game's details by Steam App ID.
#[tauri::command]
pub async fn get_game_details(
    state: State<'_,AppState>,
    app_id: i64,
) -> Result<Option<Game>, AppError> {
    if let Some(game) = state.games.get_by_id(app_id).await? {
        return Ok(Some(game));
    }

     // Not in cache — fetch from SteamSpy and cache it
    let spy_game = state.steamspy.get_app_details(app_id).await?;
    let game = steamspy_to_game(&spy_game);
    state.games.upsert(&game).await?;

    Ok(Some(game))
}

// ─────────────────────────────────────────────
// Private helpers — not exposed to React
// (no #[tauri::command] attribute)
// ─────────────────────────────────────────────

/// Core sync logic shared by sync_games and the auto-sync on empty cache.
/// Fetches top 100 from SteamSpy, converts and stores them all.
async fn sync_games_internal(state: &AppState) -> Result<usize, AppError> {
    // Fetch top 100 from the last 2 weeks — fresh, relevant games
    let spy_games = state.steamspy.get_top100_in_2weeks(&state.limits.steam_store).await?;
    tracing::info!(count = spy_games.len(), "SteamSpy sync: games fetched");

    // Convert all SteamSpy responses to our Game model
    let games: Vec<Game> = spy_games
        .iter()
        .map(steamspy_to_game)
        .collect();

    let count = games.len();

    // Bulk upsert in a single transaction (fast!)
    state.games.upsert_many(&games).await?;
    tracing::info!(count, "SteamSpy sync: games upserted to DB");

    Ok(count)
}

/// Convert a SteamSpy API response into our local Game model.
///
/// 🦀 RUST LESSON: pure conversion functions
/// This has no side effects — takes data in, returns data out.
/// Easy to test, easy to reason about.
fn steamspy_to_game(spy: &crate::api::steamspy::SteamSpyGame) -> Game {
    use chrono::Utc;

    // Serialize tags to a JSON string for storage
    // e.g. ["Indie", "RPG", "Singleplayer"]
    let tags_json = spy
        .top_tags(10)
        .into_iter()
        .collect::<Vec<_>>();
    let tags_string = serde_json::to_string(&tags_json).ok();

    Game {
        app_id:           spy.appid,
        name:             spy.name.clone(),
        review_score:     Some(spy.review_score()),
        total_reviews:    Some(spy.positive + spy.negative),
        is_indie:         spy.is_indie(),
        price_current:    spy.price_cents(),
        price_original:   spy.initial_price_cents(),
        platform_windows: true, // SteamSpy top100 is PC-only
        tags:             tags_string,
        last_updated:     Some(Utc::now()),
        gem_score:       None,
        owners_lower:    None,
        avg_playtime:    None,
        crawl_source:    None,
        header_image:    None,
        short_desc:      None,
        genres:          None,
    }
}

/// Score a game for recommendation ranking.
/// Returns a RecommendedGame with a computed score.
///
/// Scoring weights:
///   60% — review score (quality signal)
///   30% — review count, normalised (popularity signal, diminishing returns)
///   10% — discount (value signal)
fn score_game(game: &Game) -> RecommendedGame {
    let review_score = game.review_score.unwrap_or(0.0);
    let total_reviews = game.total_reviews.unwrap_or(0);

    // Normalise review count: log scale so 10k reviews isn't 100x better than 100
    // log10(100) = 2.0,  log10(1000) = 3.0,  log10(50000) = 4.7
    // Divide by 5.0 to get a 0.0-1.0 range for typical game review counts
    let review_popularity = if total_reviews > 0 {
        (total_reviews as f64).log10() / 5.0
    } else {
        0.0
    };

    // Discount score: 0.0 (no discount) to 1.0 (100% off)
    let discount_score = match (&game.price_current, &game.price_original) {
        (Some(current), Some(original)) if *original > 0 => {
            1.0 - (*current as f64 / *original as f64)
        }
        _ => 0.0,
    };

    // 🦀 RUST LESSON: match with guards
    // `(Some(current), Some(original)) if *original > 0` means:
    // match this pattern AND only if original > 0.
    // The `*` dereferences the reference inside Some.

    let recommendation_score =
        (review_score / 100.0) * 0.6
        + review_popularity.min(1.0) * 0.3
        + discount_score * 0.1;

    // Parse stored tags JSON back to Vec<String>
    let tags: Vec<String> = game
        .tags
        .as_ref()
        .and_then(|t| serde_json::from_str(t).ok())
        .unwrap_or_default();

    // Build discount label e.g. "-75%"
    let discount_label = match (&game.price_current, &game.price_original) {
        (Some(current), Some(original)) if original > current => {
            let pct = ((original - current) * 100) / original;
            Some(format!("-{}%", pct))
        }
        _ => None,
    };

    RecommendedGame {
        app_id:               game.app_id,
        name:                 game.name.clone(),
        review_score:         review_score,
        total_reviews:        total_reviews,
        price_current:        game.price_current,
        price_original:       game.price_original,
        tags,
        recommendation_score,
        discount_label,
    }
}