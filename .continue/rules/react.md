---
globs: "**/*.tsx"
---

# React Rules for Steam Lite

## Patterns
- Functional components only (no class components)
- Custom hooks in src/hooks/ for any reusable data fetching
- TanStack Query for all async data (no manual useEffect + fetch)
- Tailwind only — no inline styles, no CSS modules

## Naming
- Components: PascalCase, describes what it renders (GameCard, PriceChart)
- Hooks: camelCase with 'use' prefix (useWishlist, usePriceHistory)
- Event handlers: handle + Event (handleFilterChange, handleGameSelect)

## Performance
- Memoize expensive computations with useMemo
- Use react-window for lists >50 items (virtual scrolling)
- Avoid prop drilling >2 levels — use context or lift to TanStack Query cache