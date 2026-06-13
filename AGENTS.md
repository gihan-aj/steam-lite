# AGENTS.md — Steam Lite

## What this project is
A high-performance desktop app (Tauri + Rust backend + React frontend) that provides 
Steam game recommendations, price tracking, and wishlist analysis.
Target: gaming PCs. Performance is the primary non-functional requirement.

## Architecture (read before suggesting changes)
steam-lite/
├── src/                    # React frontend (TypeScript + Tailwind)
│   ├── components/         # Reusable UI components
│   ├── pages/              # Route-level components
│   ├── hooks/              # Custom React hooks (data fetching)
│   └── types/index.ts      # All shared TypeScript types
│
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC commands (called from React)
│   │   ├── api/            # External API clients (Steam, SteamSpy, ITAD)
│   │   ├── services/       # Business logic (recommendation, pricing, sync)
│   │   ├── db/             # SQLite repository and models
│   │   └── models/         # Rust data structures

## Naming conventions
- Rust: snake_case files, PascalCase structs, snake_case functions
- React: PascalCase components, camelCase hooks (useGameFetch), kebab-case files
- Services: verb-noun (PriceAnalyzer, RecommendationEngine, SyncService)
- Commands: verb-noun matching frontend call (get_recommended_games, fetch_wishlist)
- DB repos: noun + Repository (GameRepository, PriceRepository)

## Business rules (reference these for any new feature)
- All API calls must be cached in SQLite (see db/repository.rs)
- No blocking calls on the main thread — everything async
- Background sync runs on Tokio tasks, never blocks UI
- Error messages must be user-readable (wrap API errors, don't expose internals)
- Wishlist is owned by user (identified by steam_id in user_settings table)
- Price history is append-only (never delete, only add)

## Permissions for agents
### Allowed without asking
- Read any file, suggest naming, explain architecture
- Write unit tests, doc comments
- Suggest refactors with explanation

### Ask me first
- Adding new dependencies to Cargo.toml or package.json
- Creating new top-level modules
- Changing database schema
- Modifying existing API contracts (command signatures)

## Key decisions already made (don't re-suggest alternatives)
- Database: SQLite via sqlx (type-safe, compile-time checked)
- Async runtime: Tokio
- HTTP client: reqwest
- Frontend state/fetch: TanStack Query
- Styling: Tailwind CSS only (no CSS modules, no styled-components)