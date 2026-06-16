// 🦀 RUST LESSON: Shared mutable state across async tasks
//
// The challenge: multiple async tasks need to share one rate limiter.
//
// In single-threaded code you'd just use a mutable variable.
// In async/multi-threaded code, multiple tasks could try to modify
// it simultaneously — which would be a data race.
//
// Rust prevents data races at COMPILE TIME using:
//   Arc<T>       — "Atomically Reference Counted" — lets multiple owners
//                   share the same data. Like C#'s shared references,
//                   but the compiler tracks ownership.
//   Mutex<T>     — Mutual exclusion — only one task can access the
//                   inner value at a time. Like C#'s lock() statement.
//
// Together: Arc<Mutex<T>> = safely shared, safely mutated across tasks.
//
// Cloning an Arc doesn't copy the data — it just increments a counter.
// When the last Arc is dropped, the data is freed. Zero-cost sharing.

use std::str;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;    // Tokio's async Mutex (not std::sync::Mutex)
                           // 🦀 Key difference: std Mutex BLOCKS the thread
                           //    Tokio Mutex YIELDS to other async tasks
                           //    Always use Tokio's in async code
/// Token bucket rate limiter.
/// Allows up to `capacity` requests, refilling at `refill_rate` per interval.
#[derive(Debug)]
struct TokenBucket{
    /// Current available tokens
    tokens: f64,
    /// Maximum tokens (burst capacity)
    capacity: f64,
    /// How many tokens to add per refill
    refill_amount: f64,
    /// How often to refill
    refill_interval: Duration,
    /// When we last refilled
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: f64, refill_amount: f64, refill_interval: Duration) -> Self {
        TokenBucket {
            tokens: capacity,   // start full
            capacity,
            refill_amount,
            refill_interval,
            last_refill: Instant::now(),
        }
    }

    fn refill(&mut self){
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);

        // How many intervals has passed since the last refill?
        let intervals = elapsed.as_secs_f64() / self.refill_interval.as_secs_f64();

        if intervals >= 1.0 {
            let new_tokens = intervals * self.refill_amount;
            // Don't exceed capacity (cap the bucket)
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_refill = now;
        }
    }

    /// Try to consume one token
    /// Return how long to wait if no tokens are available
    fn try_consume(&mut self) -> Result<(), Duration> {
        self.refill();

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            Ok(())
        } 
        else {
            // Calculate how long to wait for a token
            let tokens_needed = 1.0 - self.tokens;
            let wait = self.refill_interval
                .mul_f64(tokens_needed / self.refill_amount);
            Err(wait)
        }
    }
}

// ─────────────────────────────────────────────
// PUBLIC API
// ─────────────────────────────────────────────

/// A rate limiter that can be safely shared across async tasks.
///
/// Clone it freely — all clones share the same underlying bucket.
///
/// 🦀 RUST LESSON: #[derive(Clone)] on Arc<Mutex<T>>
/// Cloning an Arc just increments the reference count.
/// All clones point to the same Mutex<TokenBucket>.
/// This is how we share one limiter across the whole app.
#[derive(Clone, Debug)]
pub struct RateLimiter {
    bucket: Arc<Mutex<TokenBucket>>,
    name: String // For debug logging
}

impl RateLimiter {
    /// Create a new rate limiter.
    ///
    /// `capacity`        — max burst (tokens available immediately)
    /// `requests_per_sec` — sustained rate
    pub fn new(name: &str, capacity: u32, requests_per_sec: f64) -> Self {
        // Convert req/sec to token refill rate
        // e.g. 5 req/sec → refill 1 token every 200ms
        let refill_interval = Duration::from_secs_f64(1.0 / requests_per_sec);

        RateLimiter {
            bucket : Arc::new(Mutex::new(TokenBucket::new(
                capacity as f64, 
                1.0, 
                refill_interval,
            ))),
            name: name.to_string(),
        }
    }

    /// Wait until a request slot is available, then return.
    ///
    /// This is the main method you call before every API request:
    ///   limiter.acquire().await;
    ///   // safe to make request now
    ///
    /// 🦀 RUST LESSON: async methods in impl blocks
    /// `async fn` inside impl works exactly like regular async functions.
    /// The caller must `.await` it.
    pub async fn acquire(&self){
        loop {
            // Lock the mutex to access the bucket
            // 🦀 .lock().await suspends this task if another task holds the lock
            //    (unlike std::Mutex which BLOCKS the thread)
            //    The lock is released when `guard` is dropped at end of scope
            let wait = {
                let mut guard = self.bucket.lock().await;
                match guard.try_consume() {
                    Ok(()) => {
                        // Got a token - proceed immediately
                        println!("[RateLimit] {} — got token, proceeding immediately", self.name);
                        return;
                    }
                    Err(wait_time) => wait_time,
                }
                // 🦀 guard is dropped HERE (end of block)
                // This releases the Mutex BEFORE we sleep
                // Critical: we must NOT hold the lock while sleeping
                // or other tasks can't check the bucket
            };

            println!(
                "[RateLimit] {} — waiting {:.0}ms",
                self.name,
                wait.as_millis()
            );

            // Yield this task for the wait duration
            // Other tasks run freely while we sleep
            tokio::time::sleep(wait).await;
        }
    }

    /// Acquire multiple tokens at once (for batch operations).
    pub async fn acquire_many(&self, count: u32) {
        for _ in 0..count {
            self.acquire().await;
        }
    }
}

// ─────────────────────────────────────────────
// PRE-CONFIGURED LIMITERS FOR EACH API
// ─────────────────────────────────────────────

/// All rate limiters for the app, grouped in one place.
/// Added to AppState so any command can access them.
#[derive(Clone, Debug)]
pub struct ApiRateLimiters {
    /// Steam Store API: ~200 requests per 5 minutes = ~0.67/sec
    /// We use 0.5/sec (1 per 2 seconds) to be conservative
    pub steam_store: RateLimiter,

    /// SteamSpy: undocumented, community-observed ~1/sec
    /// We use 0.8/sec to be safe
    pub steamspy: RateLimiter,

    /// IsThereAnyDeal: free tier is 1 req/sec
    pub itad: RateLimiter,
}

impl ApiRateLimiters {
    pub fn new() -> Self {
        ApiRateLimiters {
            steam_store: RateLimiter::new(
                "SteamStore",
                3,    // burst up to 3 requests immediately
                0.5,  // then 1 per 2 seconds sustained
            ),
            steamspy: RateLimiter::new(
                "SteamSpy",
                2,    // burst 2
                0.8,  // then 0.8/sec sustained
            ),
            itad: RateLimiter::new(
                "ITAD",
                1,    // no burst — strict 1/sec
                1.0,
            ),
        }
    }
}