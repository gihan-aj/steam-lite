import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { WishlistItem } from "../types";

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
