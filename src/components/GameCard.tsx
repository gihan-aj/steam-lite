import { formatPrice, RecommendedGame } from "../types";

// Colour accents per tag — cards feel alive without needing real images yet
const TAG_COLOURS: Record<string, string> = {
  'Action':      '#ef4444',
  'RPG':         '#8b5cf6',
  'Strategy':    '#3b82f6',
  'Puzzle':      '#f59e0b',
  'Horror':      '#6b21a8',
  'Platformer':  '#10b981',
  'Simulation':  '#06b6d4',
  'Adventure':   '#f97316',
  'Indie':       '#3d6ef8',
  'Casual':      '#ec4899',
};

function accentForGame(tags: string[]): string {
  for (const tag of tags) {
    if (TAG_COLOURS[tag]) return TAG_COLOURS[tag];
  }
  return "#3d6ef8";
}

interface GameCardProps {
  game: RecommendedGame;
}

export function GameCard({game} : GameCardProps){
  const accent = accentForGame(game.tags);
  const hasDiscount = game.discount_label != null;
  const scoreColour =
    game.review_score >= 90
      ? "#4ade80"
      : game.review_score >= 70
        ? "#f59e0b"
        : "#f87171";
  
  return (
    <article
      className="rounded-xl overflow-hidden flex flex-col transition-transform
                 duration-150 hover:-translate-y-0.5 cursor-pointer selectable"
      style={{
        background: "#1c1e27",
        border: "1px solid #242736",
      }}
      onMouseEnter={(e) => {
        (e.currentTarget as HTMLElement).style.borderColor = accent + "55";
      }}
      onMouseLeave={(e) => {
        (e.currentTarget as HTMLElement).style.borderColor = "#242736";
      }}
    >
      {/* Accent strip + image placeholder */}
      <div
        className="h-24 flex items-center justify-center relative"
        style={{
          background: `linear-gradient(135deg, ${accent}22 0%, ${accent}08 100%)`,
          borderBottom: `1px solid ${accent}33`,
        }}
      >
        {/* Top accent line */}
        <div
          className="absolute top-0 left-0 right-0 h-0.5"
          style={{ background: accent }}
        />
        {/* Game name initial as placeholder — replaced by real image later */}
        <span
          className="text-4xl font-bold opacity-20 select-none"
          style={{ color: accent }}
        >
          {game.name.charAt(0)}
        </span>

        {/* Discount badge */}
        {hasDiscount && (
          <span
            className="absolute top-2 right-2 text-xs font-bold px-1.5 py-0.5 rounded"
            style={{ background: "#16a34a", color: "#dcfce7", fontSize: 11 }}
          >
            {game.discount_label}
          </span>
        )}
      </div>

      {/* Card body */}
      <div
        className="p-3 flex flex-col gap-1 flex-1"
        style={{
          padding: "0.5rem"
        }}
      >
        {/* Name */}
        <h3
          className="font-medium leading-tight"
          style={{ fontSize: 13, color: "#e0e2e8" }}
          title={game.name}
        >
          {game.name.length > 28 ? game.name.slice(0, 26) + "…" : game.name}
        </h3>

        {/* Score + reviews */}
        <div className="flex items-center gap-1.5">
          <span style={{ fontSize: 12, color: scoreColour, fontWeight: 600 }}>
            {Math.round(game.review_score)}%
          </span>
          <span style={{ fontSize: 11, color: "#5a5f72" }}>
            {game.total_reviews.toLocaleString()} reviews
          </span>
        </div>

        {/* Price row */}
        <div className="flex items-center gap-1.5 mt-auto">
          {game.price_original && game.price_original !== game.price_current ? (
            <>
              <span
                style={{
                  fontSize: 11,
                  color: "#5a5f72",
                  textDecoration: "line-through",
                }}
              >
                {formatPrice(game.price_original)}
              </span>
              <span style={{ fontSize: 13, color: "#e0e2e8", fontWeight: 600 }}>
                {formatPrice(game.price_current)}
              </span>
            </>
          ) : (
            <span style={{ fontSize: 13, color: "#e0e2e8", fontWeight: 600 }}>
              {formatPrice(game.price_current)}
            </span>
          )}
        </div>

        {/* Tags */}
        {game.tags.length > 0 && (
          <div className="flex flex-wrap gap-1">
            {game.tags.slice(0, 3).map((tag) => (
              <span
                key={tag}
                className="px-1.5 py-0.5 rounded text-xs"
                style={{
                  background: "#242736",
                  color: "#6b7280",
                  fontSize: 10,
                  border: "1px solid #2e3145",
                }}
              >
                {tag}
              </span>
            ))}
          </div>
        )}
      </div>
    </article>
  );
}