import { formatPrice, RecommendedGame } from "../types";

interface GameCardProps {
  game: RecommendedGame;
}

export function GameCard({game} : GameCardProps){
    const hasDiscount = game.discount_label != null;
    const scoreColor =
      game.review_score >= 90
        ? "text-green-400"
        : game.review_score >= 70
          ? "text-yellow-400"
          : "text-red-400";

    return (
      <div className="bg-gray-800 rounded-lg p-4 flex flex-col gap-2 hover:bg-gray-700 transition-colors">
        {/* Game name + discount badge */}
        <div className="flex items-start justify-between gap-2">
          <h3 className="text-white font-medium text-sm leading-tight">
            {game.name}
          </h3>
          {hasDiscount && (
            <span className="bg-green-600 text-white text-xs font-bold px-2 py-0.5 rounded shrink-0">
              {game.discount_label}
            </span>
          )}
        </div>

        {/* Review score */}
        <div className="flex items-center gap-1">
          <span className={`text-sm font-semibold ${scoreColor}`}>
            {Math.round(game.review_score)}%
          </span>
          <span className="text-gray-400 text-xs">
            ({game.total_reviews.toLocaleString()} reviews)
          </span>
        </div>

        {/* Price */}
        <div className="flex items-center gap-2">
          {game.price_original && game.price_current !== game.price_original ? (
            <>
              <span className="text-gray-400 text-xs line-through">
                {formatPrice(game.price_original)}
              </span>
              <span className="text-white text-sm font-semibold">
                {formatPrice(game.price_current)}
              </span>
            </>
          ) : (
            <span className="text-white text-sm">
              {formatPrice(game.price_current)}
            </span>
          )}
        </div>

        {/* Tags */}
        {game.tags.length > 0 && (
          <div className="flex flex-wrap gap-1 mt-1">
            {game.tags.slice(0, 4).map((tag) => (
              <span
                key={tag}
                className="bg-gray-700 text-gray-300 text-xs px-2 py-0.5 rounded"
              >
                {tag}
              </span>
            ))}
          </div>
        )}
      </div>
    );
}