import { DiscoverGame, formatPrice } from "../types";

const GENRE_COLOURS: Record<string, string> = {
  Action: "#ef4444",
  RPG: "#8b5cf6",
  Strategy: "#3b82f6",
  Puzzle: "#f59e0b",
  Horror: "#6b21a8",
  Platformer: "#10b981",
  Simulation: "#06b6d4",
  Adventure: "#f97316",
  Indie: "#3d6ef8",
  Casual: "#ec4899",
};

function accentForGenres(genres: string | null): string {
  if (!genres) return "#3d6ef8";
  try {
    const parsed: string[] = JSON.parse(genres);
    for (const g of parsed) {
      if (GENRE_COLOURS[g]) return GENRE_COLOURS[g];
    }
  } catch (_) {}
  return "#3d6ef8";
}

function parseGenres(genres: string | null, max: number = 3): string[] {
  if (!genres) return [];

  try {
    const parsed: string[] = JSON.parse(genres);
    const known = parsed.filter((g) => GENRE_COLOURS[g]);
    const unknown = parsed.filter((g) => !GENRE_COLOURS[g]);
    return [...known, ...unknown].slice(0, max);
  } catch (_) {
    return [];
  }
}

function formatPlaytime(minutes: number | null): string | null {
  if (!minutes || minutes < 60) return null;
  const hours = Math.round(minutes / 60);
  return `~${hours}h avg`;
}

function formatOwners(lower: number | null): string | null {
  if (!lower) return null;
  if (lower >= 1_000_000) return `${(lower / 1_000_000).toFixed(1)}M owners`;
  if (lower >= 1_000) return `${Math.round(lower / 1000)}k owners`;
  return `${lower} owners`;
}

interface DiscoverGameCardProps {
  game: DiscoverGame;
}

export function DiscoverGameCard({ game }: DiscoverGameCardProps) {
  const accent = accentForGenres(game.genres);
  const hasDiscount =
    game.price_original !== null &&
    game.price_current !== null &&
    game.price_original > game.price_current;

  const reviewScore = game.review_score ?? 0;
  const scoreColor =
    reviewScore >= 95
      ? "#4ade80"
      : reviewScore >= 90
        ? "#86efac"
        : reviewScore >= 80
          ? "#a3e635"
          : "#fbbf24";

  const gemPct =
    game.gem_score !== null ? Math.round(game.gem_score * 100) : null;

  const genreTags = parseGenres(game.genres, 3);

  const playtime = formatPlaytime(game.avg_playtime);
  const ownership = formatOwners(game.owners_lower);

  const discountPct =
    hasDiscount && game.price_original && game.price_current
      ? Math.round((1 - game.price_current / game.price_original) * 100)
      : null;

  return (
    <article
      style={{
        background: "#1c1e27",
        border: "1px solid #242736",
        borderRadius: 12,
        overflow: "hidden",
        display: "flex",
        flexDirection: "column",
        height: "100%",
        minHeight: 300,
        transition: "border-color 0.15s, transform 0.15s",
        cursor: "default",
      }}
      onMouseEnter={(e) => {
        const el = e.currentTarget as HTMLElement;
        el.style.borderColor = `${accent}88`;
        el.style.transform = "translateY(-1px)";
      }}
      onMouseLeave={(e) => {
        const el = e.currentTarget as HTMLElement;
        el.style.borderColor = "#242736";
        el.style.transform = "translateY(0)";
      }}
    >
      {/* ── Header image ─────────────────────────── */}
      <div
        style={{
          position: "relative",
          height: 110,
          flexShrink: 0,
          overflow: "hidden",
          borderRadius: "12px 12px 0 0",
        }}
      >
        {game.header_image ? (
          <img
            src={game.header_image}
            alt={game.name}
            style={{
              width: "100%",
              height: "100%",
              objectFit: "cover",
              display: "block",
            }}
          />
        ) : (
          // Accent gradient placeholder while image not yet fetched
          <div
            style={{
              width: "100%",
              height: "100%",
              background: `linear-gradient(135deg, ${accent}22 0%, ${accent}08 100%)`,
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
            }}
          >
            <span
              style={{
                fontSize: 36,
                fontWeight: 700,
                color: accent,
                opacity: 0.25,
              }}
            >
              {game.name.charAt(0)}
            </span>
          </div>
        )}

        {/* Accent top line */}
        <div
          style={{
            position: "absolute",
            top: 0,
            left: 0,
            right: 0,
            height: 2,
            background: accent,
          }}
        />

        {/* Gradient fade */}
        <div
          style={{
            position: "absolute",
            bottom: 0,
            left: 0,
            right: 0,
            height: 48,
            background: "linear-gradient(to top, #1c1e27, transparent)",
          }}
        />

        {/* Discount badge */}
        {discountPct && (
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
            -{discountPct}%
          </div>
        )}

        {/* Gem score badge */}
        {gemPct !== null && (
          <div
            style={{
              position: "absolute",
              top: 8,
              left: 8,
              background: "rgba(0,0,0,0.7)",
              border: `1px solid ${accent}66`,
              color: accent,
              fontSize: 10,
              fontWeight: 600,
              padding: "2px 7px",
              borderRadius: 4,
            }}
          >
            ✦ {gemPct}
          </div>
        )}

        {/* Genre tags — float over the gradient fade area */}
        {genreTags.length > 0 && (
          <div
            style={{
              position: "absolute",
              bottom: 0,
              left: 0,
              right: 0,
              display: "flex",
              justifyContent: "left",
              gap: 5,
              padding: "0 8px",
            }}
          >
            {genreTags.map((genre) => {
              const colour = GENRE_COLOURS[genre] ?? accent;
              return (
                <span
                  key={genre}
                  style={{
                    fontSize: 10,
                    fontWeight: 600,
                    color: colour,
                    background: `${colour}18`,
                    border: `1px solid ${colour}44`,
                    borderRadius: 4,
                    padding: "2px 7px",
                    backdropFilter: "blur(4px)",
                    WebkitBackdropFilter: "blur(4px)",
                    letterSpacing: "0.03em",
                    textTransform: "uppercase" as const,
                  }}
                >
                  {genre}
                </span>
              );
            })}
          </div>
        )}
      </div>

      {/* ── Body ─────────────────────────────────── */}
      <div
        style={{
          padding: "10px 12px 12px",
          display: "flex",
          flexDirection: "column",
          gap: 5,
          flex: 1,
        }}
      >
        {/* Name */}
        <h3
          style={{
            fontSize: 13,
            fontWeight: 600,
            color: "#e0e2e8",
            margin: 0,
            lineHeight: 1.3,
          }}
        >
          {game.name.length > 32 ? game.name.slice(0, 30) + "…" : game.name}
        </h3>

        {/* Description */}
        {game.short_desc && (
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
            {game.short_desc}
          </p>
        )}

        {/* Review score */}
        {game.review_score !== null && (
          <div style={{ display: "flex", alignItems: "center", gap: 5 }}>
            <span
              style={{
                width: 6,
                height: 6,
                borderRadius: "50%",
                background: scoreColor,
                flexShrink: 0,
              }}
            />
            <span style={{ fontSize: 11, color: scoreColor, fontWeight: 500 }}>
              {Math.round(reviewScore)}%
            </span>
            {game.total_reviews !== null && (
              <span style={{ fontSize: 10, color: "#3a3f58" }}>
                (
                {game.total_reviews >= 1000
                  ? `${(game.total_reviews / 1000).toFixed(1)}k`
                  : game.total_reviews}{" "}
                reviews)
              </span>
            )}
          </div>
        )}

        {/* Stats row — playtime + owners */}
        {(playtime || ownership) && (
          <div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
            {playtime && (
              <span
                style={{
                  fontSize: 10,
                  color: "#4a5070",
                  background: "#1a1d28",
                  border: "1px solid #242736",
                  padding: "1px 6px",
                  borderRadius: 4,
                }}
              >
                {playtime}
              </span>
            )}
            {ownership && (
              <span
                style={{
                  fontSize: 10,
                  color: "#4a5070",
                  background: "#1a1d28",
                  border: "1px solid #242736",
                  padding: "1px 6px",
                  borderRadius: 4,
                }}
              >
                {ownership}
              </span>
            )}
          </div>
        )}

        {/* Price — pushed to bottom */}
        <div
          style={{
            marginTop: "auto",
            paddingTop: 8,
            borderTop: "1px solid #1a1d28",
          }}
        >
          {hasDiscount && game.price_original ? (
            <div style={{ display: "flex", alignItems: "baseline", gap: 6 }}>
              <span
                style={{
                  fontSize: 11,
                  color: "#5a5f72",
                  textDecoration: "line-through",
                }}
              >
                {formatPrice(game.price_original)}
              </span>
              <span style={{ fontSize: 14, fontWeight: 700, color: "#4ade80" }}>
                {formatPrice(game.price_current)}
              </span>
            </div>
          ) : (
            <span style={{ fontSize: 14, fontWeight: 700, color: "#e0e2e8" }}>
              {formatPrice(game.price_current)}
            </span>
          )}
        </div>
      </div>
    </article>
  );
}