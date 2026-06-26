import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { PricePoint, WishlistItem } from "../types";
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

const WISHLIST_KEY = ["wishlist"] as const;

export function useWishlist() {
  return useQuery({
    queryKey: WISHLIST_KEY,
    queryFn: () => invoke<WishlistItem[]>("get_wishlist"),
    staleTime: 5 * 60 * 1000,
  });
}

export function useFetchWishlist() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => invoke<WishlistItem[]>("fetch_wishlist"),
    onSuccess: (data) => {
      // Put the fresh data directly into the cache
      queryClient.setQueryData(WISHLIST_KEY, data);
    },
  });
}

export function useEnrichWishlist() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => invoke<WishlistItem[]>("enrich_wishlist_prices"),
    onSuccess: (data) => {
      queryClient.setQueryData(["wishlist"], data);
    },
  });
}

export function useGamePriceHistory(appId: number | null) {
  return useQuery({
    queryKey: ["price_history", appId],
    queryFn: () => invoke<PricePoint[]>("get_game_price_history", { appId }),
    enabled: appId != null, // only fetch when a game is selected
    staleTime: 5 * 60 * 1000,
  });
}

export function useSyncListener() {
  const queryClient = useQueryClient();

  useEffect(() => {
    // Listen for background sync completing
    const unlistenRefresh = listen("prices_refreshed", (event) => {
      const payload = event.payload as {
        updated: number;
        changed: number;
        source?: string;
      };
      console.log(
        `[SYNC] Prices refreshed: ${payload.changed} changed (${payload.source ?? "manual"})`,
      );
      // Invalidate queries so UI refreshes
      queryClient.invalidateQueries({ queryKey: ["wishlist"] });
      queryClient.invalidateQueries({ queryKey: ["settings"] });
    });

    // Listen for startup sync check
    const unlistenDue = listen("sync_due", async () => {
      console.log("[SYNC] Startup: sync due, running refresh...");
      try {
        await invoke("refresh_prices", { force: false });
      } catch (e) {
        console.error("[SYNC] Startup refresh failed:", e);
      }
    });

    // Cleanup listeners when component unmounts
    return () => {
      unlistenRefresh.then((fn) => fn());
      unlistenDue.then((fn) => fn());
    };
  }, [queryClient]);
}
