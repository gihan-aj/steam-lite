// src/pages/Wishlist.tsx
import {
  useWishlist,
  useFetchWishlist,
  useEnrichWishlist,
} from "../hooks/useWishlist";
import { useSettings } from "../hooks/useSettings";
import { WishlistItem, formatPrice } from "../types";

export function Wishlist() {
  const { data: settings } = useSettings();
  const { data: items, isLoading } = useWishlist();
  const fetch = useFetchWishlist();
  const enrich = useEnrichWishlist();

  const hasSteamId = !!settings?.steam_id;

  return (
    <div className="flex flex-col h-full">
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
              ? `${items.length} games on your wishlist`
              : "Track prices for your wishlisted games"}
          </p>
        </div>

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
            cursor: !hasSteamId || fetch.isPending ? "not-allowed" : "pointer",
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
              gap: 12,
            }}
          >
            {items.map((item) => (
              <WishlistCard key={item.app_id} item={item} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

function WishlistCard({ item }: { item: WishlistItem }) {
  const hasDiscount = item.discount_percent && item.discount_percent > 0;

  const scoreColor =
    (item.reviews_percent ?? 0) >= 90
      ? "#4ade80"
      : (item.reviews_percent ?? 0) >= 70
        ? "#f59e0b"
        : "#9096a8";

  return (
    <article
      style={{
        background: "#1c1e27",
        border: "1px solid #242736",
        borderRadius: 12,
        overflow: "hidden",
        display: "flex",
        flexDirection: "column",
        transition: "border-color 0.15s, transform 0.15s",
        cursor: "pointer",
      }}
      onMouseEnter={(e) => {
        const el = e.currentTarget as HTMLElement;
        el.style.borderColor = "#3d6ef8AA";
        el.style.transform = "translateY(-1px)";
      }}
      onMouseLeave={(e) => {
        const el = e.currentTarget as HTMLElement;
        el.style.borderColor = "#242736";
        el.style.transform = "translateY(0)";
      }}
    >
      {/* Header image */}
      <div style={{ position: "relative", height: 120, flexShrink: 0 }}>
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
          // Fallback gradient when no image
          <div
            style={{
              width: "100%",
              height: "100%",
              background: "linear-gradient(135deg, #1e2540 0%, #1a1d28 100%)",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              fontSize: 32,
              color: "#3d6ef8",
              fontWeight: 700,
              opacity: 0.4,
            }}
          >
            {item.name.charAt(0)}
          </div>
        )}

        {/* Discount badge over image */}
        {hasDiscount && (
          <div
            style={{
              position: "absolute",
              top: 8,
              right: 8,
              background: "#16a34a",
              color: "#dcfce7",
              fontSize: 12,
              fontWeight: 700,
              padding: "3px 8px",
              borderRadius: 6,
            }}
          >
            -{item.discount_percent}%
          </div>
        )}

        {/* Dark gradient overlay at bottom of image — lets text overlay look clean */}
        <div
          style={{
            position: "absolute",
            bottom: 0,
            left: 0,
            right: 0,
            height: 48,
            background: "linear-gradient(to top, #1c1e27 0%, transparent 100%)",
          }}
        />
      </div>

      {/* Card body */}
      <div
        style={{
          padding: "10px 14px 14px",
          display: "flex",
          flexDirection: "column",
          gap: 6,
          flex: 1,
        }}
      >
        {/* Game name */}
        <h3
          style={{
            fontSize: 14,
            fontWeight: 600,
            color: "#e0e2e8",
            lineHeight: 1.3,
            margin: 0,
          }}
        >
          {item.name}
        </h3>

        {/* Short description */}
        {item.short_description && (
          <p
            style={{
              fontSize: 11,
              color: "#6b7280",
              lineHeight: 1.5,
              margin: 0,
              // Clamp to 2 lines
              display: "-webkit-box",
              WebkitLineClamp: 2,
              WebkitBoxOrient: "vertical" as const,
              overflow: "hidden",
            }}
          >
            {item.short_description}
          </p>
        )}

        {/* Review score */}
        {item.review_summary && (
          <div style={{ display: "flex", alignItems: "center", gap: 4 }}>
            <span
              style={{
                width: 6,
                height: 6,
                borderRadius: "50%",
                background: scoreColor,
                flexShrink: 0,
              }}
            />
            <span style={{ fontSize: 11, color: scoreColor }}>
              {item.review_summary}
            </span>
          </div>
        )}

        {/* Spacer pushes price to bottom */}
        <div style={{ flex: 1 }} />

        {/* Price row */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            marginTop: 4,
            paddingTop: 8,
            borderTop: "1px solid #242736",
          }}
        >
          <div style={{ display: "flex", alignItems: "center", gap: 6 }}>
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
                <span
                  style={{
                    fontSize: 15,
                    fontWeight: 700,
                    color: "#4ade80",
                  }}
                >
                  {formatPrice(item.current_price)}
                </span>
              </>
            ) : (
              <span
                style={{
                  fontSize: 15,
                  fontWeight: 700,
                  color: "#e0e2e8",
                }}
              >
                {formatPrice(item.current_price)}
              </span>
            )}
          </div>

          {/* Historical low badge — when we have it */}
          {item.historical_low &&
            item.current_price &&
            item.current_price <= item.historical_low && (
              <span
                style={{
                  fontSize: 10,
                  fontWeight: 600,
                  color: "#fbbf24",
                  background: "#2d1f02",
                  border: "1px solid #78350f",
                  padding: "2px 6px",
                  borderRadius: 4,
                }}
              >
                ★ Lowest ever
              </span>
            )}
        </div>
      </div>
    </article>
  );
}
