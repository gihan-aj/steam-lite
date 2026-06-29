import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { CrawlProgress, CrawlState, DiscoverGame } from "../types";
import { useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";

export function useCrawlState() {
  return useQuery({
    queryKey: ["crawl_state"],
    queryFn: () => invoke<CrawlState>("get_crawl_state"),
    // Poll every 5 seconds while crawl might be running
    // (in addition to event-based updates)
    refetchInterval: 5000,
    staleTime: 0,
  });
}

export function useStartCrawl() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => {
      console.log("Starting crawl...");
      return invoke<void>("start_crawl");
    },
    onSuccess: () => {
      // Optimistically flip the cached status to "running" immediately
      // so the UI responds without waiting for a DB round-trip.
      // start_crawl writes Running to the DB before returning, so
      // a real refetch shortly after will confirm the same value.
      queryClient.setQueryData(
        ["crawl_state"],
        (old: CrawlState | undefined) =>
          old ? { ...old, status: "running" as CrawlState["status"] } : old,
      );
      // Real refetch after a short delay for full truth
      setTimeout(() => {
        queryClient.invalidateQueries({ queryKey: ["crawl_state"] });
      }, 800);
    },
  });
}

export function useStopCrawl() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => {
      console.log("Stopping crawl...");
      return invoke<void>("stop_crawl");
    },
    onSuccess: () => {
      // Optimistically show paused — the backend will confirm via event
      queryClient.setQueryData(
        ["crawl_state"],
        (old: CrawlState | undefined) =>
          old ? { ...old, status: "paused" as CrawlState["status"] } : old,
      );
      setTimeout(() => {
        queryClient.invalidateQueries({ queryKey: ["crawl_state"] });
      }, 800);
    },
  });
}

export function useResetCrawl() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => invoke<void>("reset_crawl"),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["crawl_state"] });
      queryClient.invalidateQueries({ queryKey: ["hidden_gems"] });
    },
  });
}

// ── Live crawl progress via Tauri events ─────────────────────────

export function useCrawlProgress() {
  const queryClient = useQueryClient();
  const [liveProgress, setLiveProgress] = useState<CrawlProgress | null>(null);
  const [countdown, setCountdown] = useState<number | null>(null);
  const countdownRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const clearCountdown = () => {
    if (countdownRef.current !== null) {
      clearInterval(countdownRef.current);
      countdownRef.current = null;
    }
    setCountdown(null);
  };

  useEffect(() => {
    const unlisten = listen<CrawlProgress>("crawl_progress", (event) => {
      const p = event.payload;
      setLiveProgress(p);

      // Also invalidate the crawl_state query so the
      // persistent state stays in sync
      queryClient.invalidateQueries({ queryKey: ["crawl_state"] });

      if (p.status === "waiting" && p.wait_seconds != null) {
        // Start local countdown - one tick per second
        clearCountdown();
        setCountdown(p.wait_seconds);
        countdownRef.current = setInterval(() => {
          setCountdown((prev) => {
            if (prev === null || prev <= 1) {
              clearInterval(countdownRef.current!);
              countdownRef.current = null;
              return null;
            }
            return prev - 1;
          });
        }, 1000);
      } else {
        // ANy non-waiting event clears the countdown
        clearCountdown();
      }

      if (p.status === "paused" || p.status === "complete") {
        queryClient.invalidateQueries({ queryKey: ["hidden_gems"] });
      }
    });

    return () => {
      unlisten.then((fn) => fn());
      clearCountdown();
    };
  }, [queryClient]);

  return { liveProgress, countdown };
}

// ── Hidden gems ───────────────────────────────────────────────────

export function useHiddenGems(limit: number = 100) {
  return useQuery({
    queryKey: ['hidden_gems', limit],
    queryFn:  () => {
        console.log('Fetching hidden gems...')
        return invoke<DiscoverGame[]>('get_hidden_gems', { limit });
    },
    staleTime: 5 * 60 * 1000,
  });
}