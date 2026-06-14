import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { RecommendedGame } from "../types";
import { invoke } from "@tauri-apps/api/core";

const GAMES_KEY = ["games", "reccomended"] as const;

export function useRecommendedGames(minScore: number = 85.0) {
  return useQuery({
    queryKey: [...GAMES_KEY, minScore],
    queryFn: async () => {
      // invoke() calls our Rust #[tauri::command] function directly.
      // The second argument maps to the Rust function parameters.
      // Note: Tauri converts snake_case params to camelCase automatically
      // so `min_score` in Rust becomes `minScore` here.
      const games = await invoke<RecommendedGame[]>("get_recommended_games", {
        minScore,
        limit: 100,
      });
      console.log(games);
      return games;
    },
    // Keep data fresh for 5 minutes before background refetch
    staleTime: 5 * 60 * 1000,
    // Show stale data while fetching fresh data in background
    placeholderData: (prev) => prev,
  });
}

// Separate hook for triggering a manual sync
export function useSyncGames() {
    const queryClient = useQueryClient();

    return useMutation({
      mutationFn: async () => {
        const count = await invoke<number>("sync_games");
        return count;
      },
      onSuccess: (count) => {
        console.log(`Synced ${count} games`);
        // Invalidate the games cache so the list refreshes automatically
        queryClient.invalidateQueries({ queryKey: ["games"] });
      },
    });
}