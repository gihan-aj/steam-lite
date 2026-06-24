// The core insight this module implements:
//
// Steam applies discount percentages globally and uniformly.
// A 65% sale in the US is also a 65% sale in Sri Lanka.
// The dollar amounts differ because regional base prices differ.
//
// ITAD tracks the highest discount % Steam has ever applied (via overview?shops=61).
// Steam's API gives us the real regional base price (via cc=lk).
//
// Therefore:
//   predicted_regional_low = regional_base_price × (1 - max_steam_cut / 100)
//
// This number is what a user in LK would actually pay at Steam's best ever sale.
// ITAD cannot compute this correctly because they don't have LK regional prices.
// We can, because we fetch from Steam directly with cc=lk.

use crate::models::{WishlistItem, PriceSignal};

/// Compute the predicted regional historical low price.
///
/// Returns None if we don't have enough data yet
/// (no regional base price or no Steam historical cut from ITAD).
pub fn compute_predicted_regional_low(item : &WishlistItem) -> Option<i64> {
    let original = item.original_price?;       // regional base price in cents
    let steam_cut = item.steam_historical_cut?; // best % Steam ever discounted

    if steam_cut <= 0 || steam_cut >= 100 {
        return None;
    }

    // Apply the discount percentage to the regional base price
    // Use integer arithmetic to avoid floating point rounding issues
    // e.g. 3499 * (100 - 65) / 100 = 3499 * 35 / 100 = 122465 / 100 = 1224
    let predicted = original * (100 - steam_cut) / 100;
    Some(predicted)
}

/// Determine if the current price is at or below the predicted regional low.
/// A small tolerance of 2 cents handles floating point edge cases.
pub fn is_at_regional_low(item: &WishlistItem) -> bool {
    match (item.current_price, item.historical_low) {
        (Some(current), Some(predicted)) => current <= predicted + 2,
        _ => false,
    }
}

/// Compute savings vs what ITAD shows (if ITAD price would be wrong).
/// Returns (itad_price, our_price, savings) if there's a meaningful difference.
///
/// ITAD uses US pricing for LK (country=lk) — their database doesn't have
/// LK regional data. So their price = US_base * (1 - cut/100).
/// Our price = LK_base * (1 - cut/100).
/// The difference is the regional pricing gap.
pub fn compute_itad_discrepancy(
    our_current: Option<i64>,
    itad_steam_current: Option<i64>,
) -> Option<i64> {
    match (our_current, itad_steam_current) {
        (Some(ours), Some(itad)) if itad > ours + 5 => {
            // ITAD shows a higher price than what Steam actually charges us
            // Threshold of 5 cents avoids showing trivial differences
            Some(itad - ours)
        }
        _ => None,
    }
}

pub fn compute_price_signal(item: &WishlistItem) -> PriceSignal {
    let at_low       = is_at_regional_low(item);
    let has_history  = item.steam_historical_cut.is_some();
    let current_cut  = item.discount_percent.unwrap_or(0);
    let best_cut     = item.steam_historical_cut.unwrap_or(0);

    // print for debugging with game name
    println!("Item: {}, at_low: {}, has_history: {}, current_cut: {}, best_cut: {}", item.name, at_low, has_history, current_cut, best_cut);

    if at_low {
        return PriceSignal {
            badge:  "★ Buy Now — Regional Low".into(),
            level:  "green".into(),
            detail: item.steam_historical_cut.map(|cut| {
                let date = item.steam_historical_date
                    .as_deref()
                    .unwrap_or("previously");
                format!("This is Steam's best ever price for your region (-{}%, {})", cut, date)
            }),
        };
    }

    if current_cut > 0 && has_history {
        if current_cut >= best_cut - 5 {
            return PriceSignal {
                badge:  "↑ Near Regional Low".into(),
                level:  "yellow".into(),
                detail: Some(format!(
                    "On sale -{current_cut}% · Regional low needs -{best_cut}% — close!"
                )),
            };
        }
        return PriceSignal {
            badge:  format!("On Sale -{current_cut}%"),
            level:  "blue".into(),
            detail: Some(format!(
                "Good deal but better is possible · Wait for -{best_cut}% sale"
            )),
        };
    }

    if current_cut > 0 {
        return PriceSignal {
            badge:  format!("On Sale -{current_cut}%"),
            level:  "blue".into(),
            detail: Some("Enrich prices to see if this is a good deal".into()),
        };
    }

    if has_history {
        let date = item.steam_historical_date.as_deref().unwrap_or("previously");
        return PriceSignal {
            badge:  "Wait for Sale".into(),
            level:  "none".into(),
            detail: Some(format!(
                "Full price · Steam has gone -{best_cut}% before ({date}) · Not on sale now"
            )),
        };
    }

    PriceSignal {
        badge:  "No History Yet".into(),
        level:  "none".into(),
        detail: Some("Click ★ Enrich to fetch price history from ITAD".into()),
    }
}

/// Compute Steam's review label from score percentage and review count.
/// Mirrors Steam's actual algorithm exactly.
///
/// 🦀 RUST LESSON: match with multiple conditions (guards)
/// We use nested match + if guards to express the two-dimensional
/// lookup table (score% × review count) cleanly.
pub fn compute_review_label(score: i64, count: i64) -> &'static str {
    match (score, count) {
        // Overwhelmingly Positive/Negative (need 500+ reviews)
        (95..=100, 500..) => "Overwhelmingly Positive",
        (0..=19,   500..) => "Overwhelmingly Negative",

        // Very Positive/Negative (need 50+ reviews)
        (85..=100, 50..) => "Very Positive",
        (0..=19,   50..) => "Very Negative",

        // Standard labels (need 10+ reviews)
        (80..=100, 10..) => "Positive",
        (70..=79,  10..) => "Mostly Positive",
        (40..=69,  10..) => "Mixed",
        (20..=39,  10..) => "Mostly Negative",
        (0..=19,   10..) => "Negative",

        // Not enough reviews
        _ => "No Reviews",
    }
    // 🦀 RUST LESSON: range patterns in match
    // 95..=100 means "95 through 100 inclusive"
    // 500.. means "500 or more" (open-ended range)
    // Rust checks patterns top to bottom and stops at first match
    // so order matters — Overwhelmingly must come before Very, etc.
}