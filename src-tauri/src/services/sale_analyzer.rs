// Detects sale patterns from price history records.
// All pure functions — no API calls, no DB access.
// Input: Vec of (timestamp, cut%) pairs
// Output: SalePattern with interval, depth, predictions

use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone)]
pub struct SalePattern {
    /// Average days between sales (None if < 2 sales observed)
    pub avg_interval_days:  Option<i64>,
    /// Lowest discount % observed
    pub typical_min_cut:    Option<i64>,
    /// Highest discount % observed (best sale ever)
    pub typical_max_cut:    Option<i64>,
    /// Date of most recent sale start
    pub last_sale_date:     Option<String>,
    /// Predicted month of next sale e.g. "2026-09"
    pub predicted_next:     Option<String>,
    /// Total number of distinct sale events observed
    pub sale_count:         i64,
}

/// A single price point with just what we need for analysis
#[derive(Debug, Clone)]
pub struct PricePoint {
    pub timestamp: DateTime<Utc>,
    pub cut:       i64,
}

/// Analyse a sorted (oldest first) list of price points.
/// Detects sale events (cut > 0) and computes patterns.
pub fn analyze_sale_pattern(points: &[PricePoint]) -> SalePattern {
    // Extract only actual sale events (cut > 0)
    // and deduplicate consecutive same-cut entries
    // (ITAD records price when it changes, so a 2-week sale
    //  appears as one entry at start and one at end when it returns to 0)
    let sale_starts: Vec<&PricePoint> = points
        .windows(2)
        .filter_map(|w| {
            // A sale "starts" when cut goes from 0 (or None) to > 0
            let prev_cut = w[0].cut;
            let curr_cut = w[1].cut;
            if curr_cut > 0 && prev_cut == 0 {
                Some(&w[1])
            }
            else {
                None
            }
        })
        .collect();

    // Also include the first point if it's already a sale
    let mut all_sales: Vec<&PricePoint> = Vec::new();
    if let Some(first) = points.first() {
        if first.cut > 0 {
            all_sales.push(first);
        }
    }
    all_sales.extend(sale_starts);

    let sale_count = all_sales.len() as i64;

    if all_sales.is_empty() {
        return SalePattern {
            avg_interval_days: None,
            typical_min_cut:   None,
            typical_max_cut:   None,
            last_sale_date:    None,
            predicted_next:    None,
            sale_count:        0,
        };
    }

    // Min/max discount across all sales
    let typical_min_cut = all_sales.iter().map(|p| p.cut).min();
    let typical_max_cut = all_sales.iter().map(|p| p.cut).max();

    // Last sale date
    let last_sale = all_sales.last().unwrap();
    let last_sale_date = Some(
        last_sale.timestamp.format("%Y-%m").to_string()
    );

    // Average interval between sales
    // 🦀 RUST LESSON: windows(2) on a slice
    // .windows(2) gives overlapping pairs: [a,b], [b,c], [c,d]...
    // Perfect for computing differences between consecutive items

    let avg_interval_days = if all_sales.len() >= 2 {
        let intervals : Vec<i64> = all_sales
            .windows(2)
            .map(|w| {
                let diff = w[1].timestamp.signed_duration_since(w[0].timestamp);
                diff.num_days().abs()
            })
            .collect();

        let total: i64 = intervals.iter().sum();
        Some(total/ intervals.len() as i64)
    }else{
        None
    };

    // Predict next sale: last sale date + average interval
    let predicted_next = avg_interval_days.map(|interval| {
        let next = last_sale.timestamp + Duration::days(interval);
        if next > Utc::now() {
            next.format("%Y-%m").to_string()
        } else {
            // Overdue — predict based on today + half interval
            let next_from_now = Utc::now() + Duration::days(interval / 2);
            next_from_now.format("%Y-%m").to_string()
        }
    });

    SalePattern {
        avg_interval_days,
        typical_min_cut,
        typical_max_cut,
        last_sale_date,
        predicted_next,
        sale_count,
    }
}