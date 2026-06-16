import { useState } from "react";
import { useRecommendedGames, useSyncGames } from "../hooks/useRecommendedGames";
import { GameCard } from "../components/GameCard";

export function Discover() {
    const [minScore, setMinScore] = useState(85);
    const {
      data: games,
      isLoading,
      isError,
      error,
    } = useRecommendedGames(minScore);
    const sync = useSyncGames();

    return (
      <div className="flex flex-col h-full">
        {/* Page header */}
        <div
          className="flex items-center justify-between px-5 py-4 shrink-0"
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
              Discover
            </h1>
            <p style={{ fontSize: 12, color: "#5a5f72", marginTop: 2 }}>
              Top-rated indie games
            </p>
          </div>

          <div className="flex items-center gap-3">
            {/* Score filter */}
            <div className="flex items-center gap-2">
              <span style={{ fontSize: 12, color: "#5a5f72" }}>Score ≥</span>
              <input
                type="range"
                min={50}
                max={99}
                step={1}
                value={minScore}
                onChange={(e) => setMinScore(Number(e.target.value))}
                className="w-24"
                style={{ accentColor: "#3d6ef8" }}
              />
              <span
                className="w-8 text-center font-semibold"
                style={{ fontSize: 13, color: "#e0e2e8" }}
              >
                {minScore}%
              </span>
            </div>

            {/* Sync button */}
            <button
              onClick={() => sync.mutate()}
              disabled={sync.isPending}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg
                       transition-colors font-medium"
              style={{
                background: sync.isPending ? "#1e2540" : "#3d6ef8",
                color: "#fff",
                padding: "7px 14px",
                fontSize: 13,
                fontWeight: 500,
                border: "none",
                cursor: sync.isPending ? "not-allowed" : "pointer",
                opacity: sync.isPending ? 0.7 : 1,
                paddingLeft: "0.5rem",
                paddingRight: "0.5rem",
              }}
            >
              <span style={{ fontSize: 14 }}>↻</span>
              {sync.isPending ? "Syncing…" : "Sync"}
            </button>
          </div>
        </div>

        {/* Sync success banner */}
        {sync.isSuccess && (
          <div
            className="mx-5 mt-3 px-3 py-2 rounded-lg flex items-center gap-2 shrink-0"
            style={{
              background: "#14291e",
              border: "1px solid #166534",
              color: "#4ade80",
              fontSize: 13,
            }}
          >
            ✓ Synced {sync.data} games successfully
          </div>
        )}

        {/* Content area */}
        <div className="flex-1 overflow-y-auto px-5 py-4">
          {/* Loading */}
          {isLoading && (
            <div className="flex flex-col items-center justify-center h-64 gap-3">
              <div
                className="w-8 h-8 rounded-full border-2 border-t-transparent animate-spin"
                style={{
                  borderColor: "#3d6ef8",
                  borderTopColor: "transparent",
                }}
              />
              <span style={{ fontSize: 13, color: "#5a5f72" }}>
                Loading games…
              </span>
            </div>
          )}

          {/* Error */}
          {isError && (
            <div
              className="p-4 rounded-xl"
              style={{ background: "#2a1515", border: "1px solid #7f1d1d" }}
            >
              <p style={{ fontSize: 13, color: "#fca5a5", fontWeight: 500 }}>
                Failed to load games
              </p>
              <p style={{ fontSize: 12, color: "#9ca3af", marginTop: 4 }}>
                {String(error)}
              </p>
            </div>
          )}

          {/* Empty state */}
          {!isLoading && games?.length === 0 && (
            <div className="flex flex-col items-center justify-center h-64 gap-3">
              <span style={{ fontSize: 40 }}>🎮</span>
              <p style={{ fontSize: 14, color: "#5a5f72" }}>
                No games found — try syncing or lowering the score filter
              </p>
              <button
                onClick={() => sync.mutate()}
                className="px-4 py-2 rounded-lg"
                style={{
                  background: "#3d6ef8",
                  color: "#fff",
                  fontSize: 13,
                  border: "none",
                  cursor: "pointer",
                }}
              >
                Sync now
              </button>
            </div>
          )}

          {/* Game count */}
          {games && games.length > 0 && (
            <p style={{ fontSize: 12, color: "#5a5f72", marginBottom: 12 }}>
              {games.length} games
            </p>
          )}

          {/* Grid */}
          {games && games.length > 0 && (
            <div
              className="grid gap-3"
              style={{
                gridTemplateColumns: "repeat(auto-fill, minmax(180px, 1fr))",
              }}
            >
              {games.map((game) => (
                <GameCard key={game.app_id} game={game} />
              ))}
            </div>
          )}
        </div>
      </div>
    );
}