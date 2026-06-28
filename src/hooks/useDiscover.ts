import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { CrawlProgress, CrawlState, DiscoverGame } from "../types";
import { useEffect, useState } from "react";
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
            queryClient.invalidateQueries({ queryKey: ["crawl_state"] });
        },
    });
}

export function useStopCrawl() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: () => {
            console.log("Stopping crawl...")
            return invoke<void>("stop_crawl");
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["crawl_state"] });
        },
    });
}

export function useResetCrawl() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: () => invoke<void>('reset_crawl'),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["crawl_state"]});
            queryClient.invalidateQueries({ queryKey: ["hidden_gems"]});
        }
    })
}

// ── Live crawl progress via Tauri events ─────────────────────────

export function useCrawlProgress() {
    const queryClient = useQueryClient();
    const [liveProgress, setLiveProgress] = useState<CrawlProgress | null>(null);

    useEffect(() => {
        const unlisten = listen<CrawlProgress>('crawl_progress', (event) => {
          setLiveProgress(event.payload);

          // Also invalidate the crawl_state query so the
          // persistent state stays in sync
          queryClient.invalidateQueries({ queryKey: ["crawl_state"] });

          // When paused, refresh the hidden gems list
          if (event.payload.status === "paused") {
            queryClient.invalidateQueries({ queryKey: ["hidden_gems"] });
          }

          // When complete, refresh the hidden gems list
          if (event.payload.status === "complete") {
            queryClient.invalidateQueries({ queryKey: ["hidden_gems"] });
          }
        });

        return () => { unlisten.then(fn => fn()); };
    }, [queryClient]);

    return liveProgress;
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