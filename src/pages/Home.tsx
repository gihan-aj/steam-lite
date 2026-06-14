import { useState } from "react";
import {
  useRecommendedGames,
  useSyncGames,
} from "../hooks/useRecommendedGames";
import { GameCard } from "../components/GameCard";

export function Home(){
    const [minScore, setMinScore] = useState(85);
    const {
      data: games,
      isLoading,
      isError,
      error,
    } = useRecommendedGames(minScore);
    const sync = useSyncGames();

    return (
      <div className="min-h-screen bg-gray-900 text-white p-6">
        {/* Header */}
        <div className="flex items-center justify-between mb-6">
          <div>
            <h1 className="text-2xl font-bold">Steam Lite</h1>
            <p className="text-gray-400 text-sm">Discover great games</p>
          </div>

          <button
            onClick={() => sync.mutate()}
            disabled={sync.isPending}
            className="bg-blue-600 hover:bg-blue-500 disabled:bg-gray-600 
                     text-white px-4 py-2 rounded-lg text-sm font-medium 
                     transition-colors"
          >
            {sync.isPending ? "Syncing..." : "Sync Games"}
          </button>
        </div>

        {/* Sync feedback */}
        {sync.isSuccess && (
          <div
            className="bg-green-900/50 border border-green-700 rounded-lg 
                        p-3 mb-4 text-green-300 text-sm"
          >
            ✓ Synced {sync.data} games successfully
          </div>
        )}

        {/* Filter */}
        <div className="flex items-center gap-3 mb-6">
          <label className="text-gray-400 text-sm">Min review score:</label>
          <input
            type="range"
            min={50}
            max={99}
            value={minScore}
            onChange={(e) => setMinScore(Number(e.target.value))}
            className="w-32"
          />
          <span className="text-white text-sm font-semibold w-10">
            {minScore}%
          </span>
        </div>

        {/* States */}
        {isLoading && (
          <div className="text-gray-400 text-center py-20">
            Loading games...
          </div>
        )}

        {isError && (
          <div className="bg-red-900/50 border border-red-700 rounded-lg p-4 text-red-300">
            <p className="font-medium">Error loading games</p>
            <p className="text-sm mt-1">{String(error)}</p>
          </div>
        )}

        {/* Game grid */}
        {games && games.length > 0 && (
          <>
            <p className="text-gray-400 text-sm mb-4">
              {games.length} games found
            </p>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
              {games.map((game) => (
                <GameCard key={game.app_id} game={game} />
              ))}
            </div>
          </>
        )}

        {games && games.length === 0 && !isLoading && (
          <div className="text-center py-20 text-gray-400">
            <p>No games found. Try lowering the minimum score or syncing.</p>
          </div>
        )}
      </div>
    );
}