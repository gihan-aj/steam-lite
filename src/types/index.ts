export interface Game {
  app_id: number;
  name: string;
  review_score: number | null;
  total_reviews: number | null;
  is_indie: boolean;
  price_current: number | null;
  price_original: number | null;
  platform_windows: boolean;
  tags: string | null;
  last_updated: string | null;
}

export interface RecommendedGame {
  app_id: number;
  name: string;
  review_score: number;
  total_reviews: number;
  price_current: number | null;
  price_original: number | null;
  tags: string[];
  recommendation_score: number;
  discount_label: string | null;
}

export interface WishlistItem {
  app_id: number;
  name: string;
  review_summary: string | null;
  reviews_percent: number | null;
  reviews_total: number | null;
  date_added: number | null;
  current_price: number | null;
  original_price: number | null;
  historical_low: number | null;
  discount_percent: number | null;
  buy_recommendation: BuyRecommendation | null;
  header_image: string | null;
  short_description: string | null;
  steam_historical_cut: number | null;
  steam_historical_date: string | null;
  all_time_low_cut: number | null;
  all_time_low_shop: string | null;
  all_time_low_date: string | null;
  predicted_regional_low: number | null;
  is_at_regional_low: boolean;
  price_signal: PriceSignal | null;
  itad_discrepancy: number | null;
  steam_review_score: number | null;
  steam_review_count: number | null;
  review_label: string | null;
  opencritic_score: number | null;
  metacritic_score: number | null;
  release_date: string | null;
  tags: string[];
  developers: string[];
  itad_id: string | null;
  avg_sale_interval_days: number | null;
  typical_discount_min: number | null;
  typical_discount_max: number | null;
  last_sale_date: string | null;
  predicted_next_sale: string | null;
}

export interface PriceSignal {
  badge: string;
  level: "green" | "yellow" | "blue" | "none";
  detail: string | null;
}

export interface PricePoint {
  app_id: number;
  price: number;
  discount_percent: number;
  recorded_at: string; // ISO timestamp string
  source: string;
}

export interface ChartPoint {
  /** Epoch milliseconds — used as the numeric X-axis value for accurate spacing */
  ts: number;
  /** Human-readable label for ticks and tooltips, e.g. "Jun '26" */
  date: string;
  /** Discount percentage, e.g. 65 */
  cut: number;
  /** Regional price in cents at this event */
  regional_price: number;
  source: string;
}

export interface BuyRecommendation {
  action: string;
  wait_score: number;
  reason: string;
  predicted_next_sale: string | null;
}

export interface UserSettings {
  steam_id: string | null;
  steam_api_key: string | null;
  itad_api_key: string | null;
  country_code: string;
  min_review_score: number;
  min_discount_percent: number;
  sync_interval_hours: number;
  last_synced_at: string | null;
  alert_threshold_percent: number;
}

// Utility: format cents as a price string
// e.g. 999 → "$9.99", null → "Free"
export function formatPrice(cents: number | null): string {
  if (cents === null || cents === 0) return "Free";
  return `$${(cents / 100).toFixed(2)}`;
}

// Utility: format review score as a label
// e.g. 95.5 → "95%"
export function formatReviewScore(score: number): string {
  return `${Math.round(score)}%`;
}

export function getReviewColor(
  label: string | null,
  score: number | null,
): string {
  if (label) {
    if (label.includes("Overwhelmingly Positive")) return "#4ade80"; // bright green
    if (label.includes("Very Positive")) return "#86efac"; // softer green
    if (label.includes("Positive") && !label.includes("Mostly"))
      return "#a3e635"; // lime
    if (label.includes("Mostly Positive")) return "#fbbf24"; // amber
    if (label.includes("Mixed")) return "#f97316"; // orange
    if (label.includes("Mostly Negative")) return "#f87171"; // soft red
    if (label.includes("Negative")) return "#ef4444"; // red
    if (label.includes("Overwhelmingly Negative")) return "#dc2626"; // deep red
  }
  // Fallback to raw score if no label yet
  if (score === null) return "#9096a8";
  if (score >= 95) return "#4ade80";
  if (score >= 85) return "#86efac";
  if (score >= 80) return "#a3e635";
  if (score >= 70) return "#fbbf24";
  if (score >= 40) return "#f97316";
  if (score >= 20) return "#f87171";
  return "#ef4444";
}

export function getReviewDisplay(item: WishlistItem): {
  label: string;
  color: string;
} {
  // Prefer the computed label, fall back to raw percentage
  const label = item.review_label ?? item.review_summary;
  const score = item.steam_review_score ?? item.reviews_percent;
  const color = getReviewColor(label, score);
  const count = item.steam_review_count ?? item.reviews_total;

  const countStr = count
    ? count >= 1000
      ? `${(count / 1000).toFixed(1)}k`
      : count.toString()
    : null;

  const displayLabel = label ?? (score ? `${score}%` : "No reviews");

  return {
    label: countStr ? `${displayLabel} · ${countStr}` : displayLabel,
    color,
  };
}

export function timeAgo(isoString: string | null): string {
  if (!isoString) return "never";
  const diff = Date.now() - new Date(isoString).getTime();
  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);

  if (minutes < 1) return "just now";
  if (minutes < 60) return `${minutes}m ago`;
  if (hours < 24) return `${hours}h ago`;
  return `${days}d ago`;
}
