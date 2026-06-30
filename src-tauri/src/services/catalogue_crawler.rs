use tokio::sync::watch;
use chrono::Utc;
use tauri::{AppHandle, Manager, Emitter};

use crate::AppState;
use crate::error::Result;
use crate::models::{CrawlState, CrawlStatus, Game};
use crate::api::steamspy::SteamSpyGame;

// ─────────────────────────────────────────────
// HIDDEN GEM SCORING
// ─────────────────────────────────────────────

/// Parse the lower bound from SteamSpy's owner range string.
/// e.g. "50,000 .. 100,000" → 50000
///      "1,000,000 .. 2,000,000" → 1000000
fn parse_owner_lower(owners: &str) -> u64 {
    owners
        .split(" ..")
        .next()
        .unwrap_or("0")
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or(0)
}

/// Score a game as a "hidden gem".
/// Returns 0.0 if the game doesn't qualify at all.
/// Returns 0.0–1.0 for qualifying games (higher = better gem).
///
/// Formula:
///   quality     (50%) — review score percentage
///   hiddenness  (35%) — inverse of owner count (fewer owners = more hidden)
///   engagement  (15%) — average playtime (people play good games longer)
pub fn hidden_gem_score(game: &SteamSpyGame) -> f64 {
    let total = game.positive + game.negative;

    // Hard disqualifiers — return 0 immediately
    if total < 50 {
        return 0.0; // not enough reviews to be meaningful
    }

    let quality = game.positive as f64 / total as f64;

    if quality < 0.80 {
        return 0.0; // Below 80% positive - not good enough
    }

    let owners = parse_owner_lower(game.owners.as_deref().unwrap_or("0"));

    if owners < 1_000 {
        return 0.0; // too obscure — probably abandoned or broken
    }

    if owners > 2_000_000 {
        return 0.0; // too popular — not hidden anymore
    }

    // ── Quality score ─────────────────────────────
    // Penalise scores below 90% more heavily
    // 95% → 0.90,  90% → 0.81,  85% → 0.72,  80% → 0.64
    let quality_score = quality.powf(2.0);

    // ── Hiddenness score ──────────────────────────
    // log10(1000)     = 3.0 → very hidden:  6.0 - 3.0 = 3.0
    // log10(100000)   = 5.0 → somewhat hidden: 6.0 - 5.0 = 1.0
    // log10(2000000)  = 6.3 → just under threshold, score ≈ 0
    // Clamp to 0.0 minimum
    let hiddenness = (6.0_f64 - (owners as f64).log10()).max(0.0) / 3.0; // normalise to 1-0

    // ── Engagement score ─────────────────────────
    // Average playtime in minutes from SteamSpy
    // log10(1)    = 0.0  (0 hours recorded)
    // log10(60)   = 1.78 (1 hour)
    // log10(600)  = 2.78 (10 hours)
    // log10(6000) = 3.78 (100 hours)
    // Divide by 4.0 to normalise to roughly 0-1
    let playtme = game.average_forever.unwrap_or(0) as f64;
    let engagement = (playtme + 1.0).log10() / 4.0;

    // ── Weighted combination ──────────────────────
    let score = quality_score   * 0.50
              + hiddenness      * 0.35
              + engagement      * 0.15;

    // Round to 4 decimal places for cleaner DB storage
    (score * 10000.0).round() / 10000.0   
}

// ─────────────────────────────────────────────
// CRAWL EVENT — emitted to React after each page
// ─────────────────────────────────────────────

#[derive(serde::Serialize, Clone)]
pub struct CrawlProgress {
    pub current_page:    u32,
    pub total_pages:     u32,
    pub games_indexed:   u32,
    pub games_qualified: u32,
    pub status:          String,
    pub percent:         u8,
    pub wait_seconds:    Option<u32>,
}

// ─────────────────────────────────────────────
// THE CRAWLER
// ─────────────────────────────────────────────

/// Run the catalogue crawl.
///
/// `stop_rx` is a watch channel receiver — when the sender sends `true`,
/// the crawl saves its state and exits cleanly.
///
/// 🦀 RUST LESSON: watch channels
/// `tokio::sync::watch` is a single-producer, multi-consumer channel
/// that always holds the most recent value.
/// Perfect for "stop signal" flags — the crawler checks it between pages.
/// Unlike a Mutex<bool>, it's designed for async and doesn't block.
pub async fn run_crawl_with_handle(
    app: AppHandle,
    mut stop_rx: watch::Receiver<bool>,
) -> Result<()> {
    let state = app.state::<AppState>();

    // Load current progress — resume from where we left off
    let mut crawl = state.crawl.load().await?;

    if crawl.status == CrawlStatus::Complete {
        tracing::info!("Crawl already complete ({} pages). Reset to re-crawl.", crawl.total_pages);
        return Ok(());
    }

    state.crawl.set_status(CrawlStatus::Running).await?;
    crawl.status = CrawlStatus::Running;

    tracing::info!(
        page = crawl.current_page,
        total = crawl.total_pages,
        "Starting crawl"
    );

    const RATE_LIMIT_SECS: u64 = 62;

    if let Some(last_page_str) = &crawl.last_page_at {
        if let Ok(last_page_time) = chrono::DateTime::parse_from_rfc3339(last_page_str) {
            let elapsed_secs = Utc::now()
                .signed_duration_since(last_page_time.with_timezone(&Utc))
                .num_seconds()
                .max(0) as u64;

            if elapsed_secs < RATE_LIMIT_SECS {
                let remaining = RATE_LIMIT_SECS - elapsed_secs;

                tracing::info!(
                    remaining_secs = remaining,
                    "Resuming — honouring SteamSpy rate limit wait"
                );

                // Notify the UI so it can show a countdown
                emit_progress(&app, &crawl, crawl.current_page, "waiting", Some(remaining as u32));
                // Wait out the remainder, but respect a stop signal
                tokio::select! {
                    _ = tokio::time::sleep(tokio::time::Duration::from_secs(remaining)) => {
                        // Rate-limit gap satisfied — proceed normally
                        tracing::info!("Rate-limit wait complete — starting page fetch");
                    }
                    _ = stop_rx.changed() => {
                        if *stop_rx.borrow_and_update() {

                            tracing::info!("Stop during rate-limit wait — saving state");

                            state.crawl.set_status(CrawlStatus::Paused).await?;
                            emit_progress(&app, &crawl, crawl.current_page, "paused", None);
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    let mut page = crawl.current_page;

    loop {
        // ── Check stop signal ─────────────────────────────────────
        // 🦀 RUST LESSON: watch::Receiver::has_changed()
        // Returns true if the value changed since last checked.
        // We check this at the start of each page — so a stop request
        // takes effect within the current page's processing time (~1 min).
        if *stop_rx.borrow() {
            tracing::info!(page, "Crawl stop requested - saving state");
            state.crawl.set_status(CrawlStatus::Paused).await?;
            emit_progress(&app, &crawl, page, "paused", None);
            return Ok(());
        }

        // ── Check if complete ─────────────────────────────────────
        if page >= crawl.total_pages {
            tracing::info!(
                indexed   = crawl.games_indexed,
                qualified = crawl.games_qualified,
                "Crawl complete"
            );
            state.crawl.set_status(CrawlStatus::Complete).await?;
            crawl.status = CrawlStatus::Complete;
            emit_progress(&app, &crawl, page, "complete", None);
            return Ok(());
        }

        tracing::info!(page, total = crawl.total_pages, "Fetching page");

        // ── Fetch page from SteamSpy ──────────────────────────────
        // Rate limiter handles the 1/min restriction
        let games = match state.steamspy
            .get_all_page(page, &state.limits.steamspy)
            .await
        {
            Ok(g) => g,
            Err(e) => {
               tracing::warn!(page, error = %e, "Failed to fetch page, will retry next session");
                // Don't advance — same page will retry on next run
                state.crawl.set_status(CrawlStatus::Paused).await?;
                emit_progress(&app, &crawl, page, "paused", None);
                return Ok(()); 
            }
        };

        if games.is_empty() {
            // SteamSpy returned empty — we've hit the end
            tracing::info!(page, "Empty page — crawl complete");
            state.crawl.set_status(CrawlStatus::Complete).await?;
            // Update total_pages to actual value
            state.crawl.set("total_pages", &page.to_string()).await?;
            emit_progress(&app, &crawl, page, "complete", None);
            return Ok(());
        }

        tracing::info!(page, count = games.len(), "Processing page");

        // ── Score and filter ──────────────────────────────────────
        let mut page_indexed   = 0u32;
        let mut page_qualified = 0u32;

        for spy_game in &games {
            page_indexed += 1;

            let score = hidden_gem_score(spy_game);
            if score <= 0.0 {
                continue; // doesn't qualify
            }

            // Qualified — convert to our Game model and store
            let game = steamspy_to_game(spy_game, score);

            match state.games.upsert(&game).await {
                Ok(_)  => page_qualified += 1,
                Err(e) => tracing::warn!(
                    app_id = spy_game.appid,
                    error  = %e,
                    "Failed to store game"
                ),
            }
        }

        // Update running totals
        crawl.games_indexed   += page_indexed;
        crawl.games_qualified += page_qualified;

        tracing::info!(
            page,
            page_qualified,
            total_qualified = crawl.games_qualified,
            "Page complete"
        );

        // ── Save progress ─────────────────────────────────────────
        // This is atomic (transaction inside advance_page).
        // If the app crashes after this line, we resume from page+1.
        state.crawl.advance_page(
            page,
            crawl.games_indexed,
            crawl.games_qualified,
        ).await?;

        // Notify React of progress
        emit_progress(&app, &crawl, page + 1, "running", None);

        page += 1;

        // ── Wait for rate limit ───────────────────────────────────
        // SteamSpy /all is limited to 1 request per minute.
        // We sleep here rather than in the rate limiter because
        // this is a background crawl — being polite is more important
        // than being fast. We also check the stop signal during the wait.
        //
        // 🦀 RUST LESSON: tokio::select!
        // select! runs multiple async branches concurrently and
        // returns when the FIRST one completes.
        // Here: either the sleep finishes (keep going) or the stop
        // signal fires (exit cleanly). Whichever happens first wins.
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(62)) => {
                // Normal case - wait is done, continue to  next page
            }
            _ = stop_rx.changed() => {
                // Stop signal received during wait — exit cleanly
                if *stop_rx.borrow_and_update() {
                    tracing::info!("Stop signal during page wait — exiting");
                    state.crawl.set_status(CrawlStatus::Paused).await?;
                    emit_progress(&app, &crawl, page, "paused", None);
                    return Ok(());
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
// HELPERS
// ─────────────────────────────────────────────

fn emit_progress(
    app: &AppHandle, 
    crawl: &CrawlState, 
    current_page: u32, 
    status: &str,
    wait_seconds: Option<u32>,
) {
    let percent = if crawl.total_pages > 0 {
        ((current_page as f32 / crawl.total_pages as f32) * 100.0) as u8
    } else {
        0
    };

    let _ = app.emit("crawl_progress", CrawlProgress {
        current_page,
        total_pages:     crawl.total_pages,
        games_indexed:   crawl.games_indexed,
        games_qualified: crawl.games_qualified,
        status:          status.to_string(),
        percent,
        wait_seconds,
    });
}

/// Convert a SteamSpy game to our Game model with the gem score attached.
fn steamspy_to_game(spy: &SteamSpyGame, gem_score: f64) -> Game {
    let owners_lower = parse_owner_lower(spy.owners.as_deref().unwrap_or("0")) as i64;

    let total = spy.positive + spy.negative;
    let review_score = if total > 0 {
        Some((spy.positive as f64 / total as f64) * 100.0)
    } else {
        None
    };

    Game {
        app_id:          spy.appid,
        name:            spy.name.clone(),
        review_score,
        total_reviews:   Some(total as i64),
        is_indie:        false,    // we don't know yet — set after appdetails fetch
        price_current:   spy.price_cents(),
        price_original:  spy.initial_price_cents(),
        platform_windows: true,
        tags:            None,     // populated after appdetails fetch
        last_updated:    Some(Utc::now()),
        gem_score:       Some(gem_score),
        owners_lower:    Some(owners_lower),
        avg_playtime:    spy.average_forever.map(|p| p as i64),
        crawl_source:    Some(format!("all_page_{}", 0)),  // updated by caller
        header_image:    None,     // fetched separately (rate limited)
        short_desc:      None,
        genres:          None,
        last_price_check: None,
    }
}
