import { useEffect, useState } from "react";
import {
  useCrawlProgress,
  useCrawlState,
  useEnrichDiscoverGames,
  useHiddenGems,
  useResetCrawl,
  useStartCrawl,
  useStopCrawl,
} from "../hooks/useDiscover";
import { DiscoverGameCard } from "../components/DiscoverGameCard";
import { SkeletonGrid } from "../components/SkeletonCard";

export function Discover() {
  const [minScore, setMinScore] = useState(85);

  const { data: crawlState, isLoading: crawlLoading } = useCrawlState();
  const { liveProgress, countdown } = useCrawlProgress(); // real-time event updates
  const { data: gems, isLoading: gemsLoading } = useHiddenGems(100);

  const start = useStartCrawl();
  const stop = useStopCrawl();
  const reset = useResetCrawl();

  const liveIsStale =
    liveProgress !== null &&
    crawlState !== undefined &&
    (liveProgress.status === "paused" || liveProgress.status === "complete") &&
    crawlState.status === "running";

  // Use live progress if available, otherwise fall back to DB state
  const progress =
    liveIsStale || liveProgress === null
      ? crawlState
        ? {
            current_page: crawlState.current_page,
            total_pages: crawlState.total_pages,
            games_indexed: crawlState.games_indexed,
            games_qualified: crawlState.games_qualified,
            status: crawlState.status,
            wait_seconds: undefined,
            percent:
              crawlState.total_pages > 0
                ? Math.round(
                    (crawlState.current_page / crawlState.total_pages) * 100,
                  )
                : 0,
          }
        : null
      : liveProgress;

  const isRunning = progress?.status === "running";
  const isComplete = progress?.status === "complete";
  const isIdle = !progress || progress.status === "idle";
  const isPaused = progress?.status === "paused";
  const isWaiting = progress?.status === "waiting";

  // Filter gems by minimum review score
  const filteredGems = (gems ?? []).filter(
    (g) => (g.review_score ?? 0) >= minScore,
  );

  const enrich = useEnrichDiscoverGames();

  useEffect(() => {
    if (!gems || gems.length === 0) return;

    const needsEnrichment = gems
      .sort((a, b) => (b.gem_score ?? 0) - (a.gem_score ?? 0))
      .slice(0, 40)
      .filter((g) => !g.header_image)
      .map((g) => g.app_id);

    if (needsEnrichment.length > 0) {
      enrich.mutate(needsEnrichment);
    }
  }, [gems?.length]);

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100%" }}>
      {/* ── Page header ──────────────────────────── */}
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
            Discover
          </h1>
          <p style={{ fontSize: 12, color: "#5a5f72", marginTop: 2 }}>
            {isComplete
              ? `${progress?.games_qualified ?? 0} hidden gems found`
              : "High-quality indie games you might have missed"}
          </p>
        </div>

        {/* Score filter */}
        <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
          <span style={{ fontSize: 12, color: "#5a5f72" }}>Min score</span>
          <input
            type="range"
            min={80}
            max={99}
            step={1}
            value={minScore}
            onChange={(e) => setMinScore(Number(e.target.value))}
            style={{ width: 80, accentColor: "#3d6ef8" }}
          />
          <span
            style={{
              fontSize: 12,
              fontWeight: 600,
              color: "#e0e2e8",
              width: 32,
            }}
          >
            {minScore}%
          </span>
        </div>
      </div>

      {/* ── Crawl control panel ───────────────────── */}
      {!crawlLoading && (
        <CrawlPanel
          progress={progress}
          isRunning={isRunning}
          isComplete={isComplete}
          isIdle={isIdle}
          isPaused={isPaused}
          isWaiting={isWaiting}
          countdown={countdown}
          onStart={() => start.mutate()}
          onStop={() => stop.mutate()}
          onReset={() => reset.mutate()}
          startPending={start.isPending}
          stopPending={stop.isPending}
        />
      )}

      {/* ── Content area ─────────────────────────── */}
      <div style={{ flex: 1, overflowY: "auto", padding: "16px 20px" }}>
        {/* Loading skeletons */}
        {gemsLoading && <SkeletonGrid count={12} />}

        {/* Empty states */}
        {!gemsLoading && filteredGems.length === 0 && isIdle && (
          <EmptyState
            icon="🔍"
            title="No games catalogued yet"
            description="Start the catalogue crawl to discover hidden gems. It runs in the background — you can keep using the app while it works."
          />
        )}

        {!gemsLoading &&
          filteredGems.length === 0 &&
          (isRunning || isPaused) && (
            <EmptyState
              icon="⏳"
              title="Building your catalogue…"
              description={`Page ${progress?.current_page ?? 0} of ${progress?.total_pages ?? 50} · ${progress?.games_qualified ?? 0} gems found so far`}
            />
          )}

        {!gemsLoading && filteredGems.length === 0 && isComplete && (
          <EmptyState
            icon="🎮"
            title="No games match your filter"
            description="Try lowering the minimum review score filter above."
          />
        )}

        {/* Game grid */}
        {filteredGems.length > 0 && (
          <>
            <p style={{ fontSize: 12, color: "#5a5f72", marginBottom: 12 }}>
              {filteredGems.length} games · sorted by gem score
            </p>
            <div
              style={{
                display: "grid",
                gridTemplateColumns: "repeat(auto-fill, minmax(200px, 1fr))",
                gridAutoRows: "1fr",
                gap: 12,
              }}
            >
              {filteredGems
                .sort((a, b) => (b.gem_score ?? 0) - (a.gem_score ?? 0))
                .map((game) => (
                  <DiscoverGameCard key={game.app_id} game={game} />
                ))}
            </div>
          </>
        )}
      </div>
    </div>
  );
}

// ── Sub-components ────────────────────────────────────────────────

function CrawlPanel({
  progress,
  isRunning,
  isComplete,
  isIdle,
  isPaused,
  isWaiting,
  countdown,
  onStart,
  onStop,
  onReset,
  startPending,
  stopPending,
}: {
  progress: {
    current_page: number;
    total_pages: number;
    games_indexed: number;
    games_qualified: number;
    percent: number;
    status: string;
  } | null;
  isRunning: boolean;
  isComplete: boolean;
  isIdle: boolean;
  isPaused: boolean;
  isWaiting: boolean;
  countdown: number | null;
  onStart: () => void;
  onStop: () => void;
  onReset: () => void;
  startPending: boolean;
  stopPending: boolean;
}) {
  if (isComplete && !progress?.games_qualified) return null;

  const statusColors = {
    running: { bg: "#0f1e35", border: "#1d4ed8", text: "#60a5fa" },
    waiting: { bg: "#1a1a10", border: "#a16207", text: "#eab308" },
    paused: { bg: "#292010", border: "#ca8a04", text: "#fbbf24" },
    complete: { bg: "#14291e", border: "#16a34a", text: "#4ade80" },
    idle: { bg: "#1a1d28", border: "#2a2d3a", text: "#5a5f72" },
  };

  const status = (progress?.status ?? "idle") as keyof typeof statusColors;
  const colors = statusColors[status] ?? statusColors.idle;

  return (
    <div
      style={{
        margin: "12px 20px 0",
        padding: "12px 14px",
        background: colors.bg,
        border: `1px solid ${colors.border}`,
        borderRadius: 8,
        display: "flex",
        alignItems: "center",
        gap: 14,
        flexShrink: 0,
      }}
    >
      {/* Status dot + label */}
      <div
        style={{ display: "flex", alignItems: "center", gap: 6, flexShrink: 0 }}
      >
        <div
          style={{
            width: 7,
            height: 7,
            borderRadius: "50%",
            background: colors.text,
            // Pulse animation when running
            animation: isRunning ? "pulse 1.5s infinite" : "none",
          }}
        />
        <span style={{ fontSize: 12, fontWeight: 600, color: colors.text }}>
          {isRunning
            ? "Crawling"
            : isWaiting
              ? "Rate limit"
              : isPaused
                ? "Paused"
                : isComplete
                  ? "Complete"
                  : "Not started"}
        </span>
      </div>

      {/* Progress bar + stats */}
      {progress && !isIdle && (
        <div
          style={{ flex: 1, display: "flex", flexDirection: "column", gap: 4 }}
        >
          {/* Bar */}
          <div
            style={{
              height: 4,
              background: "#1a1d28",
              borderRadius: 2,
              overflow: "hidden",
            }}
          >
            <div
              style={{
                height: "100%",
                width: `${progress.percent}%`,
                background: colors.text,
                borderRadius: 2,
                transition: "width 0.5s ease",
              }}
            />
          </div>

          {/* Stats */}
          <div style={{ display: "flex", gap: 12 }}>
            {isWaiting && countdown !== null ? (
              <span style={{ fontSize: 11, color: colors.text, opacity: 0.9 }}>
                Next page in {countdown}s — respecting SteamSpy rate limit
              </span>
            ) : (
              <>
                <span
                  style={{ fontSize: 11, color: colors.text, opacity: 0.8 }}
                >
                  Page {progress.current_page}/{progress.total_pages} (
                  {progress.percent}%)
                </span>
                <span
                  style={{ fontSize: 11, color: colors.text, opacity: 0.6 }}
                >
                  {progress.games_qualified} gems found
                </span>
                <span
                  style={{ fontSize: 11, color: colors.text, opacity: 0.4 }}
                >
                  {progress.games_indexed.toLocaleString()} scanned
                </span>
              </>
            )}
          </div>
        </div>
      )}

      {isIdle && (
        <p style={{ fontSize: 12, color: "#5a5f72", margin: 0, flex: 1 }}>
          Scan ~50,000 Steam games to find hidden gems. Runs in background · ~50
          min total · resumable anytime.
        </p>
      )}

      {/* Buttons */}
      <div style={{ display: "flex", gap: 6, flexShrink: 0 }}>
        {(isIdle || isPaused) && !isWaiting && (
          <CrawlButton
            onClick={onStart}
            disabled={startPending}
            color="#3d6ef8"
          >
            {isPaused ? "▶ Resume" : "▶ Start"}
          </CrawlButton>
        )}

        {(isRunning || isWaiting) && (
          <CrawlButton onClick={onStop} disabled={stopPending} color="#ca8a04">
            ⏸ Pause
          </CrawlButton>
        )}

        {!isIdle && (
          <CrawlButton onClick={onReset} disabled={false} color="#3a3f58">
            ↺ Reset
          </CrawlButton>
        )}
      </div>
    </div>
  );
}

function CrawlButton({
  children,
  onClick,
  disabled,
  color,
}: {
  children: React.ReactNode;
  onClick: () => void;
  disabled: boolean;
  color: string;
}) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      style={{
        background: color,
        color: "#fff",
        border: "none",
        borderRadius: 6,
        padding: "5px 12px",
        fontSize: 12,
        fontWeight: 500,
        cursor: disabled ? "not-allowed" : "pointer",
        opacity: disabled ? 0.6 : 1,
        transition: "opacity 0.15s",
      }}
    >
      {children}
    </button>
  );
}

function EmptyState({
  icon,
  title,
  description,
}: {
  icon: string;
  title: string;
  description: string;
}) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        height: "60%",
        gap: 12,
        padding: 40,
      }}
    >
      <span style={{ fontSize: 40, opacity: 0.4 }}>{icon}</span>
      <h2
        style={{ fontSize: 15, color: "#e0e2e8", fontWeight: 500, margin: 0 }}
      >
        {title}
      </h2>
      <p
        style={{
          fontSize: 12,
          color: "#5a5f72",
          textAlign: "center",
          maxWidth: 300,
          lineHeight: 1.6,
        }}
      >
        {description}
      </p>
    </div>
  );
}
