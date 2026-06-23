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
}

export interface PriceSignal {
  badge: string;
  level: "green" | "yellow" | "blue" | "none";
  detail: string | null;
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
  min_review_score: number;
  min_discount_percent: number;
  sync_interval_hours: number;
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
