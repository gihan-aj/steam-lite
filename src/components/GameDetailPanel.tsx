import { useEffect, useRef } from "react";
import {
  ChartPoint,
  formatPrice,
  getReviewDisplay,
  WishlistItem,
} from "../types";
import { useGamePriceHistory } from "../hooks/useWishlist";
import {
  CartesianGrid,
  Line,
  LineChart,
  ReferenceLine,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";

interface GameDetailPanelProps {
  game: WishlistItem | null;
  onClose: () => void;
  regionCode?: string;
}

export function GameDetailPanel({
  game,
  onClose,
  regionCode = "LK",
}: GameDetailPanelProps) {
  const panelRef = useRef<HTMLDivElement>(null);
  const { data: priceHistory, isLoading: historyLoading } = useGamePriceHistory(
    game?.app_id ?? null,
  );

  // Close on Escape Key
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [onClose]);

  // Build step-chart data from raw price history events.
  // Each point is a discrete price-change event — we preserve exact timestamps
  // so the X-axis reflects real time gaps between sales.
  const chartData: ChartPoint[] = (() => {
    const filtered = (priceHistory ?? [])
      .filter(
        (p) =>
          p.source === "itad_history" ||
          p.source === "itad_steam" ||
          p.source === "steam_live",
      )
      // Sort chronologically so stepAfter draws correctly
      .sort(
        (a, b) =>
          new Date(a.recorded_at).getTime() - new Date(b.recorded_at).getTime(),
      );

    if (filtered.length === 0) return [];

    const points: ChartPoint[] = filtered.map((p) => {
      const ts = new Date(p.recorded_at).getTime();
      const label = new Date(p.recorded_at).toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
        year: "2-digit",
      });
      const regionalPrice = game?.original_price
        ? Math.round(game.original_price * (1 - p.discount_percent / 100))
        : 0;
      return {
        ts,
        date: label,
        cut: p.discount_percent,
        regional_price: regionalPrice,
        source: p.source,
      };
    });

    // Append a synthetic "now" point so the step extends to the right edge
    // and shows the current discount state visually
    const last = points[points.length - 1];
    const nowTs = Date.now();
    if (nowTs - last.ts > 12 * 60 * 60 * 1000) {
      // only append if last point is older than 12 h
      const nowLabel = new Date().toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
        year: "2-digit",
      });
      points.push({
        ts: nowTs,
        date: nowLabel,
        cut: game?.discount_percent ?? last.cut,
        regional_price: game?.current_price ?? last.regional_price,
        source: "now",
      });
    }

    return points;
  })();

  const review = game ? getReviewDisplay(game) : null;

  const hasMultipleReviews =
    (game?.opencritic_score !== null && game?.opencritic_score !== undefined) ||
    (game?.metacritic_score !== null && game?.metacritic_score !== undefined);

  // Slide animation: transform based on whether a game is selected
  const isOpen = game !== null;

  return (
    <>
      {/* Backdrop — clicking it closes the panel */}
      <div
        onClick={onClose}
        style={{
          position: "fixed",
          inset: 0,
          background: "transparent",
          zIndex: 40,
          // Only intercept clicks when panel is open
          pointerEvents: isOpen ? "auto" : "none",
        }}
      />

      {/* Panel */}
      <div
        ref={panelRef}
        style={{
          position: "fixed",
          top: 36, // below titlebar
          right: 0,
          bottom: 0,
          width: 420,
          background: "#13141a",
          borderLeft: "1px solid #1a1d28",
          zIndex: 50,
          display: "flex",
          flexDirection: "column",
          overflowY: "auto",
          // Slide in/out animation
          transform: isOpen ? "translateX(0)" : "translateX(100%)",
          transition: "transform 0.2s ease-out",
          // transition: "transform 0.25s cubic-bezier(0.4, 0, 0.2, 1)",
          willChange: "transform", // Hardware acceleration hint — prevents paint stutter
        }}
        // Prevent backdrop click from firing when clicking inside panel
        onClick={(e) => e.stopPropagation()}
      >
        {game && (
          <PanelContent
            game={game}
            historyLoading={historyLoading}
            chartData={chartData}
            review={review}
            onClose={onClose}
            hasMultipleReviews={hasMultipleReviews}
            regionCode={regionCode}
          />
        )}
      </div>
    </>
  );
}

// ── Panel content — separated so it only renders when game is set ──
function PanelContent({
  game,
  historyLoading,
  chartData,
  review,
  onClose,
  hasMultipleReviews,
  regionCode = "LK",
}: {
  game: WishlistItem;
  historyLoading: boolean;
  chartData: ChartPoint[];
  review: { label: string; color: string } | null;
  onClose: () => void;
  hasMultipleReviews: boolean;
  regionCode?: string;
}) {
  const hasDiscount = (game.discount_percent ?? 0) > 0;
  const hasSalePattern = game.avg_sale_interval_days != null;
  const hasTags = game.tags.length > 0;

  return (
    <>
      {/* ── Header image ───────────────────────────── */}
      <div style={{ position: "relative", height: 180, flexShrink: 0 }}>
        {game.header_image ? (
          <img
            src={game.header_image}
            alt={game.name}
            style={{ width: "100%", height: "100%", objectFit: "cover" }}
          />
        ) : (
          <div
            style={{
              width: "100%",
              height: "100%",
              background: "linear-gradient(135deg, #1e2540, #1a1d28)",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              fontSize: 48,
              color: "#3d6ef8",
              opacity: 0.3,
            }}
          >
            {game.name.charAt(0)}
          </div>
        )}

        {/* Gradient overlay */}
        <div
          style={{
            position: "absolute",
            inset: 0,
            background:
              "linear-gradient(to bottom, rgba(19,20,26,0) 30%, #13141a 100%)",
          }}
        />

        {/* Close button */}
        <button
          onClick={onClose}
          style={{
            position: "absolute",
            top: 10,
            right: 10,
            width: 28,
            height: 28,
            background: "rgba(0,0,0,0.6)",
            border: "1px solid #2a2d3a",
            borderRadius: 6,
            color: "#9096a8",
            cursor: "pointer",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            fontSize: 14,
          }}
        >
          ✕
        </button>
      </div>

      {/* ── Body ───────────────────────────────────── */}
      <div
        style={{
          padding: "0 18px 24px",
          display: "flex",
          flexDirection: "column",
          gap: 20,
        }}
      >
        {/* Game title + meta */}
        <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
          <h2
            style={{
              fontSize: 18,
              fontWeight: 700,
              color: "#e0e2e8",
              margin: 0,
            }}
          >
            {game.name}
          </h2>
          <div
            style={{
              display: "flex",
              flexWrap: "wrap",
              gap: 6,
              alignItems: "center",
            }}
          >
            {game.developers.length > 0 && (
              <span style={{ fontSize: 12, color: "#5a5f72" }}>
                {game.developers[0]}
              </span>
            )}
            {game.release_date && (
              <>
                <span style={{ color: "#2a2d3a" }}>·</span>
                <span style={{ fontSize: 12, color: "#5a5f72" }}>
                  {new Date(game.release_date).getFullYear()}
                </span>
              </>
            )}
          </div>

          {/* Tags */}
          {hasTags && (
            <div
              style={{
                display: "flex",
                flexWrap: "wrap",
                gap: 4,
                marginTop: 2,
              }}
            >
              {game.tags.slice(0, 5).map((tag) => (
                <span
                  key={tag}
                  style={{
                    fontSize: 10,
                    color: "#5a6080",
                    background: "#1a1d28",
                    border: "1px solid #242736",
                    padding: "2px 7px",
                    borderRadius: 4,
                  }}
                >
                  {tag}
                </span>
              ))}
            </div>
          )}
        </div>

        {/* ── Reviews ───────────────────────────────── */}
        <Section title="Reviews">
          <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
            {/* Steam */}
            {game.steam_review_score !== null && (
              <ReviewBar
                source="Steam"
                score={game.steam_review_score}
                label={game.review_label}
                count={game.steam_review_count}
                color={review?.color ?? "#9096a8"}
              />
            )}

            {/* OpenCritic */}
            {hasMultipleReviews && game.opencritic_score !== null && (
              <ReviewBar
                source="OpenCritic"
                score={game.opencritic_score}
                color={
                  (game.opencritic_score ?? 0) >= 85
                    ? "#4ade80"
                    : (game.opencritic_score ?? 0) >= 70
                      ? "#fbbf24"
                      : "#f87171"
                }
              />
            )}

            {/* Metacritic */}
            {hasMultipleReviews && game.metacritic_score !== null && (
              <ReviewBar
                source="Metacritic"
                score={game.metacritic_score}
                color={
                  (game.metacritic_score ?? 0) >= 75
                    ? "#4ade80"
                    : (game.metacritic_score ?? 0) >= 60
                      ? "#fbbf24"
                      : "#f87171"
                }
              />
            )}
          </div>
        </Section>

        {/* ── Your Price ────────────────────────────── */}
        <Section title={`Your Price (${regionCode} Regional)`}>
          <div style={{ display: "flex", alignItems: "baseline", gap: 8 }}>
            {hasDiscount && game.original_price ? (
              <>
                <span
                  style={{
                    fontSize: 12,
                    color: "#5a5f72",
                    textDecoration: "line-through",
                  }}
                >
                  {formatPrice(game.original_price)}
                </span>
                <span
                  style={{ fontSize: 24, fontWeight: 700, color: "#4ade80" }}
                >
                  {formatPrice(game.current_price)}
                </span>
                <span
                  style={{
                    fontSize: 12,
                    fontWeight: 700,
                    background: "#1a3a1a",
                    border: "1px solid #166534",
                    color: "#4ade80",
                    padding: "2px 7px",
                    borderRadius: 5,
                  }}
                >
                  -{game.discount_percent}%
                </span>
              </>
            ) : (
              <span style={{ fontSize: 24, fontWeight: 700, color: "#e0e2e8" }}>
                {formatPrice(game.current_price)}
              </span>
            )}
          </div>

          {/* Price signal */}
          {game.price_signal && (
            <PriceSignalBanner signal={game.price_signal} />
          )}

          {/* Regional low comparison */}
          {game.predicted_regional_low && (
            <div
              style={{
                marginTop: 8,
                display: "flex",
                flexDirection: "column",
                gap: 4,
              }}
            >
              <PriceCompareRow
                label={`Steam historical low (${regionCode})`}
                price={game.predicted_regional_low}
                cut={game.steam_historical_cut}
                date={game.steam_historical_date}
                isMatch={game.is_at_regional_low}
              />
              {game.all_time_low_shop && game.all_time_low_shop !== "Steam" && (
                <PriceCompareRow
                  label={`Other shops · ${game.all_time_low_shop}`}
                  cut={game.all_time_low_cut}
                  date={game.all_time_low_date}
                  warning={`⚠ may not ship to ${regionCode}`}
                />
              )}
            </div>
          )}
        </Section>

        {/* ── Price History Chart ───────────────────── */}
        {historyLoading ? (
          <div
            style={{
              height: 160,
              background:
                "linear-gradient(90deg, #1a1d28 25%, #242736 50%, #1a1d28 75%)",
              backgroundSize: "200% 100%",
              animation: "shimmer 1.5s infinite",
              borderRadius: 6,
            }}
          />
        ) : chartData.length > 1 ? (
          <Section title={`Steam Price History (${regionCode})`}>
            <div style={{ height: 180, marginTop: 4 }}>
              <ResponsiveContainer width="100%" height="100%">
                {/*
                  Step chart: discounts are discrete events, not gradual changes.
                  - X-axis uses epoch ms (numeric) so real time gaps are preserved.
                  - type="stepAfter" draws a horizontal plateau between events,
                    which is exactly how Steam sales look.
                */}
                <LineChart
                  data={chartData}
                  margin={{ top: 4, right: 8, bottom: 0, left: -20 }}
                >
                  <CartesianGrid
                    strokeDasharray="3 3"
                    stroke="#1a1d28"
                    vertical={false}
                  />
                  <XAxis
                    dataKey="ts"
                    type="number"
                    scale="time"
                    domain={["dataMin", "dataMax"]}
                    tickFormatter={(ms: number) =>
                      new Date(ms).toLocaleDateString("en-US", {
                        month: "short",
                        year: "2-digit",
                      })
                    }
                    tick={{ fontSize: 10, fill: "#5a5f72" }}
                    axisLine={false}
                    tickLine={false}
                    tickCount={5}
                  />
                  <YAxis
                    tickFormatter={(v) => `${v}%`}
                    tick={{ fontSize: 10, fill: "#5a5f72" }}
                    axisLine={false}
                    tickLine={false}
                    domain={[0, 100]}
                  />
                  <Tooltip
                    content={
                      <ChartTooltip
                        originalPrice={game.original_price}
                        regionCode={regionCode}
                      />
                    }
                  />
                  {/* Horizontal baseline at 0% — helps read the "not on sale" periods */}
                  <ReferenceLine y={0} stroke="#2a2d3a" strokeWidth={1} />
                  {/* Reference line at current discount */}
                  {game.discount_percent != null &&
                    game.discount_percent > 0 && (
                      <ReferenceLine
                        y={game.discount_percent}
                        stroke="#4ade80"
                        strokeDasharray="4 2"
                        strokeOpacity={0.5}
                        label={{
                          value: `now · -${game.discount_percent}%`,
                          fill: "#4ade80",
                          fontSize: 9,
                          position: "insideTopRight",
                        }}
                      />
                    )}
                  <Line
                    type="stepAfter"
                    dataKey="cut"
                    stroke="#3d6ef8"
                    strokeWidth={2}
                    dot={false}
                    activeDot={{
                      r: 4,
                      fill: "#3d6ef8",
                      stroke: "#13141a",
                      strokeWidth: 2,
                    }}
                    isAnimationActive={false}
                  />
                </LineChart>
              </ResponsiveContainer>
            </div>
            <p style={{ fontSize: 10, color: "#3a3f58", marginTop: 4 }}>
              Each step = a recorded price-change event. Hover for details.
            </p>
          </Section>
        ) : (
          <Section title={`Steam Price History (${regionCode})`}>
            <p style={{ fontSize: 12, color: "#5a5f72", padding: "12px 0" }}>
              {chartData.length === 0
                ? "No history yet — click ★ Enrich to fetch from ITAD"
                : "Only one data point recorded — more will accumulate over time"}
            </p>
          </Section>
        )}

        {/* ── Sale Pattern ──────────────────────────── */}
        {hasSalePattern && (
          <Section title="Sale Pattern Analysis">
            <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
              <StatRow
                label="Goes on sale"
                value={`every ~${game.avg_sale_interval_days} days`}
              />
              {game.typical_discount_min !== null &&
                game.typical_discount_max !== null && (
                  <StatRow
                    label="Typical discount"
                    value={
                      game.typical_discount_min === game.typical_discount_max
                        ? `-${game.typical_discount_max}%`
                        : `-${game.typical_discount_min}% to -${game.typical_discount_max}%`
                    }
                  />
                )}
              {game.last_sale_date && (
                <StatRow label="Last sale" value={game.last_sale_date} />
              )}
              {game.predicted_next_sale && (
                <StatRow
                  label="Next sale likely"
                  value={game.predicted_next_sale}
                  highlight
                />
              )}
            </div>
          </Section>
        )}

        {/* ── Short description ─────────────────────── */}
        {game.short_description && (
          <Section title="About">
            <p
              style={{
                fontSize: 12,
                color: "#6b7280",
                lineHeight: 1.6,
                margin: 0,
              }}
            >
              {game.short_description}
            </p>
          </Section>
        )}
      </div>
    </>
  );
}

// ── Sub-components ─────────────────────────────────────────────────

function Section({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div>
      <h3
        style={{
          fontSize: 10,
          fontWeight: 600,
          color: "#3a3f58",
          textTransform: "uppercase",
          letterSpacing: "0.1em",
          margin: "0 0 10px",
        }}
      >
        {title}
      </h3>
      {children}
    </div>
  );
}

function ReviewBar({
  source,
  score,
  label,
  count,
  color,
}: {
  source: string;
  score: number;
  label?: string | null;
  count?: number | null;
  color: string;
}) {
  const countStr = count
    ? count >= 1000
      ? `${(count / 1000).toFixed(1)}k reviews`
      : `${count} reviews`
    : null;

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "baseline",
        }}
      >
        <span style={{ fontSize: 11, color: "#5a5f72" }}>{source}</span>
        <div style={{ display: "flex", alignItems: "baseline", gap: 6 }}>
          {label && (
            <span style={{ fontSize: 11, color, fontWeight: 500 }}>
              {label}
            </span>
          )}
          <span style={{ fontSize: 13, fontWeight: 700, color }}>{score}%</span>
          {countStr && (
            <span style={{ fontSize: 10, color: "#3a3f58" }}>{countStr}</span>
          )}
        </div>
      </div>
      {/* Score bar */}
      <div
        style={{
          height: 3,
          background: "#1a1d28",
          borderRadius: 2,
          overflow: "hidden",
        }}
      >
        <div
          style={{
            height: "100%",
            width: `${score}%`,
            background: color,
            borderRadius: 2,
            transition: "width 0.6s ease",
          }}
        />
      </div>
    </div>
  );
}

function PriceSignalBanner({
  signal,
}: {
  signal: { badge: string; level: string; detail: string | null };
}) {
  const colors: Record<string, { bg: string; border: string; text: string }> = {
    green: { bg: "#14291e", border: "#16a34a", text: "#4ade80" },
    yellow: { bg: "#292010", border: "#ca8a04", text: "#fbbf24" },
    blue: { bg: "#0f1e35", border: "#1d4ed8", text: "#60a5fa" },
    none: { bg: "#1a1d28", border: "#2a2d3a", text: "#5a5f72" },
  };
  const c = colors[signal.level] ?? colors.none;

  return (
    <div
      style={{
        marginTop: 10,
        padding: "8px 12px",
        background: c.bg,
        border: `1px solid ${c.border}`,
        borderRadius: 7,
        display: "flex",
        flexDirection: "column",
        gap: 3,
      }}
    >
      <span style={{ fontSize: 12, fontWeight: 600, color: c.text }}>
        {signal.badge}
      </span>
      {signal.detail && (
        <span
          style={{ fontSize: 11, color: c.text, opacity: 0.8, lineHeight: 1.4 }}
        >
          {signal.detail}
        </span>
      )}
    </div>
  );
}

function PriceCompareRow({
  label,
  price,
  cut,
  date,
  isMatch,
  warning,
}: {
  label: string;
  price?: number | null;
  cut?: number | null;
  date?: string | null;
  isMatch?: boolean;
  warning?: string;
}) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: 2,
        padding: "5px 0",
        borderBottom: "1px solid #15171f",
      }}
    >
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
        }}
      >
        <span style={{ fontSize: 10, color: "#3a3f58" }}>{label}</span>
        <div style={{ display: "flex", alignItems: "center", gap: 5 }}>
          {cut !== null && cut !== undefined && (
            <span
              style={{
                fontSize: 10,
                fontWeight: 600,
                background: isMatch ? "#14291e" : "#1a1d28",
                border: `1px solid ${isMatch ? "#166534" : "#242736"}`,
                color: isMatch ? "#4ade80" : "#5a6080",
                padding: "1px 5px",
                borderRadius: 4,
              }}
            >
              -{cut}%
            </span>
          )}
          {price && (
            <span
              style={{
                fontSize: 12,
                fontWeight: 600,
                color: isMatch ? "#4ade80" : "#9096a8",
              }}
            >
              {formatPrice(price)}
            </span>
          )}
        </div>
      </div>
      {(date || warning) && (
        <div style={{ display: "flex", justifyContent: "flex-end", gap: 8 }}>
          {date && (
            <span style={{ fontSize: 10, color: "#3a3f58" }}>{date}</span>
          )}
          {warning && (
            <span
              style={{ fontSize: 10, color: "#4a5070", fontStyle: "italic" }}
            >
              {warning}
            </span>
          )}
        </div>
      )}
    </div>
  );
}

function StatRow({
  label,
  value,
  highlight,
}: {
  label: string;
  value: string;
  highlight?: boolean;
}) {
  return (
    <div
      style={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
        padding: "4px 0",
        borderBottom: "1px solid #15171f",
      }}
    >
      <span style={{ fontSize: 12, color: "#5a5f72" }}>{label}</span>
      <span
        style={{
          fontSize: 12,
          fontWeight: highlight ? 600 : 400,
          color: highlight ? "#fbbf24" : "#9096a8",
        }}
      >
        {value}
      </span>
    </div>
  );
}

// Custom recharts tooltip
function ChartTooltip({
  active,
  payload,
  originalPrice,
  regionCode = "LK",
}: {
  active?: boolean;
  payload?: Array<{ value: number; payload: ChartPoint }>;
  originalPrice?: number | null;
  regionCode?: string;
}) {
  if (!active || !payload?.length) return null;
  const point = payload[0].payload;
  const regionalPrice = originalPrice
    ? Math.round(originalPrice * (1 - point.cut / 100))
    : null;

  return (
    <div
      style={{
        background: "#1c1e27",
        border: "1px solid #2a2d3a",
        borderRadius: 6,
        padding: "7px 10px",
        display: "flex",
        flexDirection: "column",
        gap: 3,
      }}
    >
      <span style={{ fontSize: 11, color: "#9096a8" }}>{point.date}</span>
      {point.cut > 0 ? (
        <span style={{ fontSize: 13, fontWeight: 600, color: "#3d6ef8" }}>
          -{point.cut}% off
        </span>
      ) : (
        <span style={{ fontSize: 13, fontWeight: 600, color: "#5a5f72" }}>
          Full price
        </span>
      )}
      {regionalPrice !== null && regionalPrice !== undefined && (
        <span style={{ fontSize: 11, color: point.cut > 0 ? "#4ade80" : "#9096a8" }}>
          {regionCode} price: {formatPrice(regionalPrice)}
        </span>
      )}
      {point.source === "now" && (
        <span style={{ fontSize: 10, color: "#3a3f58", fontStyle: "italic" }}>current</span>
      )}
    </div>
  );
}