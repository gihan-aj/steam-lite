// src/pages/Wishlist.tsx
import { useWishlist, useFetchWishlist } from "../hooks/useWishlist";
import { useSettings } from "../hooks/useSettings";
import { WishlistItem, formatPrice } from "../types";

export function Wishlist() {
  const { data: settings } = useSettings();
  const { data: items, isLoading } = useWishlist();
  const fetch = useFetchWishlist();

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
          <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
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
    <div
      style={{
        background: "#1c1e27",
        border: "1px solid #242736",
        borderRadius: 10,
        padding: "12px 16px",
        display: "flex",
        alignItems: "center",
        gap: 16,
        transition: "border-color 0.15s",
      }}
      onMouseEnter={(e) =>
        ((e.currentTarget as HTMLElement).style.borderColor = "#3d6ef8AA")
      }
      onMouseLeave={(e) =>
        ((e.currentTarget as HTMLElement).style.borderColor = "#242736")
      }
    >
      {/* Game name + review */}
      <div style={{ flex: 1, minWidth: 0 }}>
        <div
          style={{
            fontSize: 14,
            fontWeight: 500,
            color: "#e0e2e8",
            marginBottom: 3,
          }}
        >
          {item.name}
        </div>
        {item.review_summary && (
          <div style={{ fontSize: 11, color: scoreColor }}>
            {item.review_summary}
            {item.reviews_percent ? ` · ${item.reviews_percent}%` : ""}
            {item.reviews_total
              ? ` (${item.reviews_total.toLocaleString()})`
              : ""}
          </div>
        )}
      </div>

      {/* Price block */}
      <div style={{ textAlign: "right", flexShrink: 0 }}>
        {hasDiscount ? (
          <>
            <div
              style={{
                display: "flex",
                alignItems: "center",
                gap: 6,
                justifyContent: "flex-end",
              }}
            >
              <span
                style={{
                  background: "#16a34a",
                  color: "#dcfce7",
                  fontSize: 11,
                  fontWeight: 700,
                  padding: "1px 6px",
                  borderRadius: 4,
                }}
              >
                -{item.discount_percent}%
              </span>
              <span style={{ fontSize: 14, fontWeight: 600, color: "#e0e2e8" }}>
                {formatPrice(item.current_price)}
              </span>
            </div>
            <div
              style={{
                fontSize: 11,
                color: "#5a5f72",
                textDecoration: "line-through",
                marginTop: 2,
              }}
            >
              {formatPrice(item.original_price ?? item.current_price)}
            </div>
          </>
        ) : (
          <span style={{ fontSize: 14, fontWeight: 600, color: "#e0e2e8" }}>
            {item.current_price ? formatPrice(item.current_price) : "Free"}
          </span>
        )}
      </div>
    </div>
  );
}
