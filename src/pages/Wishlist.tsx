// src/pages/Wishlist.tsx
import {
  useWishlist,
  useFetchWishlist,
  useEnrichWishlist,
} from "../hooks/useWishlist";
import { useSettings } from "../hooks/useSettings";
import { WishlistItem, formatPrice, getReviewDisplay, timeAgo } from "../types";
import { useState } from "react";
import { GameDetailPanel } from "../components/GameDetailPanel";

export function Wishlist() {
  const { data: settings } = useSettings();
  const { data: items, isLoading } = useWishlist();
  const fetch = useFetchWishlist();
  const enrich = useEnrichWishlist();

  const [selectedGame, setSelectedGame] = useState<WishlistItem | null>(null);

  const hasSteamId = !!settings?.steam_id;

  return (
    <div className="flex flex-col h-full" style={{ position: "relative" }}>
      {/* Header */}
      <div
        style={{
          padding: "16px 20px",
          borderBottom: "1px solid #1a1d28",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          flexShrink: 0,
        }}
      >
        <div>
          <h1 style={{ fontSize: 20, fontWeight: 600, color: "#e0e2e8" }}>
            Wishlist
          </h1>

          <p style={{ fontSize: 12, color: "#5a5f72", marginTop: 2 }}>
            {items?.length
              ? `${items.length} games · synced ${timeAgo(settings?.last_synced_at ?? null)}`
              : "Track prices for your wishlisted games"}
          </p>
        </div>

        <div
          style={{
            display: "flex",
            alignItems: "end",
            gap: 12,
          }}
        >
          <button
            onClick={() => fetch.mutate()}
            disabled={fetch.isPending || !hasSteamId}
            title={!hasSteamId ? "Add your Steam ID in Settings first" : ""}
            style={{
              background: hasSteamId ? "#3d6ef8" : "#1c1e27",
              color: hasSteamId ? "#fff" : "#5a5f72",
              border: hasSteamId ? "none" : "1px solid #2a2d3a",
              borderRadius: 8,
              padding: "7px 14px",
              fontSize: 13,
              fontWeight: 500,
              cursor:
                !hasSteamId || fetch.isPending ? "not-allowed" : "pointer",
              opacity: fetch.isPending ? 0.7 : 1,
            }}
          >
            {fetch.isPending ? "Fetching…" : "↻ Sync wishlist"}
          </button>

          <button
            onClick={() => enrich.mutate()}
            disabled={enrich.isPending || !hasSteamId}
            style={{
              background: "transparent",
              color: "#3d6ef8",
              border: "1px solid #3d6ef8",
              borderRadius: 8,
              padding: "7px 14px",
              fontSize: 13,
              fontWeight: 500,
              cursor: enrich.isPending ? "not-allowed" : "pointer",
              opacity: enrich.isPending ? 0.7 : 1,
            }}
          >
            {enrich.isPending ? "Fetching prices…" : "★ Enrich prices"}
          </button>
        </div>
      </div>

      {/* No Steam ID state */}
      {!hasSteamId && (
        <div
          style={{
            margin: 20,
            padding: "16px 20px",
            background: "#1a1d28",
            border: "1px solid #2a2d3a",
            borderRadius: 10,
            fontSize: 13,
            color: "#9096a8",
          }}
        >
          Add your Steam ID in{" "}
          <strong style={{ color: "#e0e2e8" }}>Settings</strong> to sync your
          wishlist. Your profile must be set to public.
        </div>
      )}

      {/* Error state */}
      {fetch.isError && (
        <div
          style={{
            margin: "0 20px 16px",
            padding: "10px 14px",
            background: "#2a1515",
            border: "1px solid #7f1d1d",
            borderRadius: 8,
            fontSize: 12,
            color: "#fca5a5",
          }}
        >
          {String(fetch.error)}
        </div>
      )}

      {/* Content */}
      <div className="flex-1 overflow-y-auto" style={{ padding: "12px 20px" }}>
        {isLoading && (
          <div className="flex items-center justify-center h-40">
            <span style={{ color: "#5a5f72", fontSize: 13 }}>Loading…</span>
          </div>
        )}

        {!isLoading && (!items || items.length === 0) && hasSteamId && (
          <div className="flex flex-col items-center justify-center h-64 gap-3">
            <span style={{ fontSize: 40 }}>❤️</span>
            <p style={{ color: "#5a5f72", fontSize: 13 }}>
              No wishlist data yet — click Sync to fetch from Steam
            </p>
          </div>
        )}

        {items && items.length > 0 && (
          <div
            style={{
              display: "grid",
              gridTemplateColumns: "repeat(auto-fill, minmax(220px, 1fr))",
              gridAutoRows: "1fr",
              gap: 12,
              // Shrink grid when panel is open — leaves room for panel
              // paddingRight: selectedGame ? 430 : 0,
              // transition: "padding-right 0.25s cubic-bezier(0.4, 0, 0.2, 1)",
            }}
          >
            {items.map((item) => (
              <div
                key={item.app_id}
                onClick={() => setSelectedGame(item)}
                style={{ cursor: "pointer" }}
              >
                <WishlistCard item={item} />
              </div>
            ))}
          </div>
        )}

        {/* Detail panel — slides in from right */}
        <GameDetailPanel
          game={selectedGame}
          onClose={() => setSelectedGame(null)}
        />
      </div>
    </div>
  );
}

function WishlistCard({
  item,
  regionCode = "LK",
}: {
  item: WishlistItem;
  regionCode?: string;
}) {
  const hasDiscount = (item.discount_percent ?? 0) > 0;
  const hasHistory = item.steam_historical_cut !== null;
  const hasOtherShop =
    item.all_time_low_shop !== null && item.all_time_low_shop !== "Steam";

  const signal = item.price_signal;

  const signalColors: Record<
    string,
    { bg: string; border: string; text: string }
  > = {
    green: { bg: "#14291e", border: "#16a34a", text: "#4ade80" },
    yellow: { bg: "#292010", border: "#ca8a04", text: "#fbbf24" },
    blue: { bg: "#0f1e35", border: "#1d4ed8", text: "#60a5fa" },
    none: { bg: "#1a1d28", border: "#2a2d3a", text: "#5a5f72" },
  };
  const sigColor = signalColors[signal?.level ?? "none"];

  const review = getReviewDisplay(item);

  const cardBorderColor = item.is_at_regional_low ? "#16a34a55" : "#242736";

  return (
    <article
      style={{
        background: "#1c1e27",
        border: `1px solid ${cardBorderColor}`,
        borderRadius: 12,
        display: "flex",
        flexDirection: "column",
        transition: "border-color 0.15s, transform 0.15s",
        cursor: "pointer",
        height: "100%",
        minHeight: 320,
      }}
      onMouseEnter={(e) => {
        const el = e.currentTarget as HTMLElement;
        el.style.borderColor = item.is_at_regional_low
          ? "#16a34a"
          : "#3d6ef8AA";
        el.style.transform = "translateY(-1px)";
      }}
      onMouseLeave={(e) => {
        const el = e.currentTarget as HTMLElement;
        el.style.borderColor = cardBorderColor;
        el.style.transform = "translateY(0)";
      }}
    >
      {/* ── Header image — overflow:hidden here clips the image to border radius ── */}
      <div
        style={{
          position: "relative",
          height: 115,
          flexShrink: 0,
          overflow: "hidden", // ← image clipping only, not card content
          borderRadius: "12px 12px 0 0", // ← top corners only
        }}
      >
        {item.header_image ? (
          <img
            src={item.header_image}
            alt={item.name}
            style={{
              width: "100%",
              height: "100%",
              objectFit: "cover",
              display: "block",
            }}
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
              fontSize: 36,
              color: "#3d6ef8",
              fontWeight: 700,
              opacity: 0.3,
            }}
          >
            {item.name.charAt(0)}
          </div>
        )}

        {/* Gradient fade */}
        <div
          style={{
            position: "absolute",
            bottom: 0,
            left: 0,
            right: 0,
            height: 56,
            background: "linear-gradient(to top, #1c1e27, transparent)",
          }}
        />

        {/* Discount badge */}
        {hasDiscount && (
          <div
            style={{
              position: "absolute",
              top: 8,
              right: 8,
              background: "#1a3a1a",
              border: "1px solid #166534",
              color: "#4ade80",
              fontSize: 11,
              fontWeight: 700,
              padding: "3px 7px",
              borderRadius: 5,
            }}
          >
            -{item.discount_percent}%
          </div>
        )}
      </div>

      {/* ── Game info ─────────────────────────────── */}
      <div
        style={{
          padding: "10px 13px 0",
          display: "flex",
          flexDirection: "column",
          gap: 5,
        }}
      >
        <h3
          style={{
            fontSize: 13,
            fontWeight: 600,
            color: "#e0e2e8",
            margin: 0,
            lineHeight: 1.3,
          }}
        >
          {item.name}
        </h3>

        {item.short_description && (
          <p
            style={{
              fontSize: 11,
              color: "#5a5f72",
              margin: 0,
              lineHeight: 1.5,
              display: "-webkit-box",
              WebkitLineClamp: 2,
              WebkitBoxOrient: "vertical" as const,
              overflow: "hidden",
            }}
          >
            {item.short_description}
          </p>
        )}

        {(item.review_summary || item.review_summary) && (
          <div style={{ display: "flex", alignItems: "center", gap: 5 }}>
            <span
              style={{
                width: 6,
                height: 6,
                borderRadius: "50%",
                background: review.color,
                flexShrink: 0,
              }}
            />
            <span style={{ fontSize: 11, color: review.color }}>
              {review.label}
            </span>
          </div>
        )}
      </div>

      {/* ── Current price ─────────────────────────── */}
      <div
        style={{
          padding: "8px 13px",
          marginTop: 6,
          borderTop: "1px solid #1a1d28",
          display: "flex",
          alignItems: "baseline",
          gap: 6,
        }}
      >
        {hasDiscount && item.original_price ? (
          <>
            <span
              style={{
                fontSize: 11,
                color: "#5a5f72",
                textDecoration: "line-through",
              }}
            >
              {formatPrice(item.original_price)}
            </span>
            <span style={{ fontSize: 17, fontWeight: 700, color: "#4ade80" }}>
              {formatPrice(item.current_price)}
            </span>
          </>
        ) : (
          <span style={{ fontSize: 17, fontWeight: 700, color: "#e0e2e8" }}>
            {formatPrice(item.current_price)}
          </span>
        )}
        <span style={{ fontSize: 10, color: "#3a3f58" }}>
          {regionCode} regional
        </span>
      </div>

      {/* ── Signal banner — BEFORE price rows so position is consistent ── */}
      {signal && (
        <div
          style={{
            margin: "0 13px 6px",
            padding: "7px 10px",
            background: sigColor.bg,
            border: `1px solid ${sigColor.border}`,
            borderRadius: 6,
            display: "flex",
            flexDirection: "column",
            gap: 2,
          }}
        >
          <span style={{ fontSize: 11, fontWeight: 600, color: sigColor.text }}>
            {signal.badge}
          </span>
          {signal.detail && (
            <span
              style={{
                fontSize: 10,
                color: sigColor.text,
                opacity: 0.8,
                lineHeight: 1.4,
              }}
            >
              {signal.detail}
            </span>
          )}
        </div>
      )}

      {/* ── Price rows — after signal, at card bottom ─── */}
      {(hasHistory || hasOtherShop) && (
        <div
          style={{
            padding: "0 13px 12px",
            display: "flex",
            flexDirection: "column",
            gap: 0,
            marginTop: "auto", // ← pushes price rows to card bottom
            borderTop: "1px solid #1a1d28",
            paddingTop: 6,
          }}
        >
          <PriceRow
            label="Steam low"
            show={hasHistory}
            cut={item.steam_historical_cut}
            date={item.steam_historical_date}
            price={item.predicted_regional_low}
            isMatch={item.is_at_regional_low}
            note={item.is_at_regional_low ? "← you are here" : undefined}
            noteColor="#4ade80"
            tooltipText={`Steam's best ever discount × your ${regionCode} regional base price`}
          />
          <PriceRow
            label="Other shops"
            show={hasOtherShop}
            cut={item.all_time_low_cut}
            date={item.all_time_low_date}
            shopName={item.all_time_low_shop}
            note={`⚠ may not ship to ${regionCode}`}
            noteColor="#4a5070"
            tooltipText="May not be available in your region or use Steam regional pricing"
          />
        </div>
      )}
    </article>
  );
}

function PriceRow({
  label,
  show,
  cut,
  date,
  price,
  shopName,
  isMatch,
  note,
  noteColor,
  tooltipText,
}: {
  label: string;
  show: boolean;
  cut: number | null;
  date: string | null;
  price?: number | null;
  shopName?: string | null;
  isMatch?: boolean;
  note?: string;
  noteColor?: string;
  tooltipText?: string;
}) {
  if (!show || cut === null) return null;

  return (
    <div
      title={tooltipText}
      style={{
        padding: "5px 0",
        borderBottom: "1px solid #15171f",
        display: "flex",
        flexDirection: "column",
        gap: 2,
      }}
    >
      {/* Line 1: label (+ shop name) on left, cut% + price on right */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          gap: 6,
        }}
      >
        {/* Left: label · shop name */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 3,
            minWidth: 0,
            flex: 1,
          }}
        >
          <span
            style={{
              fontSize: 10,
              color: "#3a3f58",
              flexShrink: 0,
              fontWeight: 500,
            }}
          >
            {label}
          </span>
          {shopName && (
            <>
              <span style={{ fontSize: 10, color: "#2a2d3a", flexShrink: 0 }}>
                ·
              </span>
              <span
                style={{
                  fontSize: 10,
                  color: "#4a5070",
                  overflow: "hidden",
                  textOverflow: "ellipsis",
                  whiteSpace: "nowrap",
                  // Takes remaining space, truncates if too long
                  minWidth: 0,
                }}
              >
                {shopName}
              </span>
            </>
          )}
        </div>

        {/* Right: cut% badge + price */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 4,
            flexShrink: 0,
          }}
        >
          <span
            style={{
              fontSize: 10,
              fontWeight: 600,
              color: isMatch ? "#4ade80" : "#5a6080",
              background: isMatch ? "#14291e" : "#1a1d28",
              border: `1px solid ${isMatch ? "#166534" : "#242736"}`,
              padding: "1px 5px",
              borderRadius: 4,
            }}
          >
            -{cut}%
          </span>

          {price !== undefined && price !== null && (
            <span
              style={{
                fontSize: 11,
                fontWeight: 600,
                color: isMatch ? "#4ade80" : "#9096a8",
              }}
            >
              {formatPrice(price)}
            </span>
          )}
        </div>
      </div>

      {/* Line 2: date + note — right-aligned under the cut%/price */}
      {(date || note) && (
        <div
          style={{
            display: "flex",
            justifyContent: "flex-end", // aligns under cut%/price above
            alignItems: "center",
            gap: 5,
          }}
        >
          {note && (
            <span
              style={{
                fontSize: 10,
                color: noteColor ?? "#5a5f72",
                fontStyle: "italic",
              }}
            >
              {note}
            </span>
          )}
          {date && (
            <span style={{ fontSize: 10, color: "#3a3f58" }}>({date})</span>
          )}
        </div>
      )}
    </div>
  );
}