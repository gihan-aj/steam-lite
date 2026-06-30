import { useEffect, useRef } from "react";

export function useIntersectionObserver(
    callback: () => void,
    options: IntersectionObserverInit = {}
) {
    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const element = ref.current;
        if (!element) return;

        const observer = new IntersectionObserver(
          (entries) => {
            // Entry is intersecting = element is visible in viewport
            if (entries[0]?.isIntersecting) {
              callback();
            }
          },
          {
            // rootMargin: trigger 200px before the sentinel reaches the viewport
            // so new content loads before the user actually hits the bottom
            rootMargin: "200px",
            threshold: 0,
            ...options,
          },
        );

        observer.observe(element);
        return () => observer.disconnect();
    }, [callback]);

    return ref;
}