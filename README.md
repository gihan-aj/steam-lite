# Steam Lite

A high-performance desktop app for discovering great Steam games, tracking wishlist prices, and finding the optimal time to buy. Built with Rust + Tauri + React as a portfolio project focused on performance-first architecture.

> **Status:** 🚧 In active development — MVP in progress

---

## Why this exists

Steam has over 50,000 games. Finding genuinely great indie games buried under paid promotions, tracking whether your wishlist items are at a historically good price, and knowing whether to buy now or wait for a better sale — none of this is easy in the default Steam UI.

Steam Lite solves this with a local-first, lightweight desktop app that pulls from public APIs, caches everything locally, and runs quietly without eating your gaming PC's resources.

---

## Features

### MVP (current focus)
- 🎮 **Indie game discovery** — Filter by review score, player count, and recency using SteamSpy data. Surfaces games with overwhelmingly positive reviews that you may have missed.
- 💸 **Discount tracker** — Find games currently at significant discounts, with context on whether the discount is historically good or just average.
- ❤️ **Wishlist importer** — Fetch your Steam wishlist directly from your public profile. No login required.
- 📈 **Price history** — View historical price data from IsThereAnyDeal to understand a game's typical sale pattern.

### Planned
- ⏱️ **Optimal buy time predictor** — Scores each wishlist game on whether to buy now or wait, based on historical sale frequency and depth.
- 🔔 **Price drop alerts** — Background daemon checks prices and notifies when a wishlist game hits your threshold.
- 💎 **Hidden gems algorithm** — High-rated, low-visibility indie games scored by review quality vs. player count ratio.
- 📊 **Wishlist ROI calculator** — "If you wait for typical sale periods, you'd save $X across your wishlist."

---

## Tech stack

| Layer | Technology | Why |
|-------|-----------|-----|
| Desktop shell | [Tauri v2](https://tauri.app/) | Native performance, ~40MB memory vs Electron's 150MB+ |
| Backend | [Rust](https://www.rust-lang.org/) + [Tokio](https://tokio.rs/) | Async-first, zero-cost abstractions, fearless concurrency |
| HTTP client | [Reqwest](https://docs.rs/reqwest) | Async HTTP with JSON support |
| Database | [SQLite](https://www.sqlite.org/) via [sqlx](https://github.com/launchbadge/sqlx) | Local-first, compile-time checked queries, zero setup |
| Frontend | [React](https://react.dev/) + [TypeScript](https://www.typescriptlang.org/) | Type-safe UI, familiar ecosystem |
| Data fetching | [TanStack Query](https://tanstack.com/query) | Caching, background refetch, stale-while-revalidate |
| Styling | [Tailwind CSS](https://tailwindcss.com/) | Minimal bundle, utility-first |

---

## Data sources

All public APIs, no authentication required for core features.

| API | Used for | Rate limit |
|-----|---------|-----------|
| [SteamSpy](https://steamspy.com/api.php) | Game stats, review scores, tags | ~1 req/sec |
| [Steam Community](https://steamcommunity.com/) | Wishlist data, game details | ~1-2 req/sec |
| [IsThereAnyDeal](https://isthereanydeal.com/) | Price history, historical lows | Free tier |

---

## Architecture

```
steam-lite/
├── src/                          # React frontend
│   ├── components/               # Reusable UI (GameCard, PriceChart, etc.)
│   ├── pages/                    # Route-level views (Home, Wishlist, Settings)
│   ├── hooks/                    # Data fetching hooks (useGameFetch, useWishlist)
│   └── types/index.ts            # Shared TypeScript types
│
└── src-tauri/                    # Rust backend
    └── src/
        ├── commands/             # Tauri IPC commands (called from React)
        ├── api/                  # External API clients (steam, steamspy, itad)
        ├── services/             # Business logic (recommendations, price analysis)
        ├── db/                   # SQLite repository layer
        └── models/               # Shared Rust data structures
```

### How the layers talk to each other

```
React (UI)
    │  Tauri IPC  invoke("get_recommended_games", { min_score: 90 })
    ▼
Rust Commands  (src-tauri/src/commands/)
    │  calls
    ▼
Services  (recommendation engine, price analyzer, sync daemon)
    │  reads/writes
    ▼
SQLite cache  (local DB — fast, offline-capable)
    ▲
Background sync  (Tokio tasks — fetches APIs, updates DB hourly/daily)
    │  fetches
    ▼
External APIs  (SteamSpy, Steam Community, IsThereAnyDeal)
```

### Performance design decisions

- **Two-layer cache:** In-memory LRU (hot data) → SQLite (persistent). API calls only happen when cache is stale.
- **Parallel fetches:** `tokio::join!` fetches SteamSpy + Steam reviews + price history simultaneously, not sequentially.
- **Incremental sync:** Background daemon only fetches games updated since last sync — not the full catalogue.
- **Virtual scrolling:** Game lists use `react-window` to render only visible items, keeping the UI at 60fps regardless of catalogue size.

---

## Getting started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) LTS
- Windows: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with "Desktop development with C++"

### Install and run

```bash
# Clone the repo
git clone https://github.com/gihan-aj/steam-lite.git
cd steam-lite

# Install frontend dependencies
npm install

# Run in development mode (starts both Rust backend and React frontend)
npm run tauri dev
```

The first build will take a few minutes — Rust compiles all dependencies from source. Subsequent builds are much faster thanks to incremental compilation.

### Build for release

```bash
npm run tauri build
# Output: src-tauri/target/release/bundle/
```

---

## Development

### Database migrations

```bash
# Install sqlx CLI (one time)
cargo install sqlx-cli --no-default-features --features sqlite

# Run migrations
sqlx migrate run --database-url sqlite:steam-lite.db
```

### Project scripts

| Command | What it does |
|---------|-------------|
| `npm run tauri dev` | Start dev mode (hot reload for React, rebuild on Rust changes) |
| `npm run tauri build` | Build release binary |
| `cargo test` | Run Rust unit and integration tests (from `src-tauri/`) |
| `npm test` | Run React component tests |

---

## Roadmap

### Phase 1 — Foundation (current)
- [x] Project scaffold (Tauri + React + Rust)
- [x] SQLite schema and migrations
- [ ] SteamSpy API client + game caching
- [ ] Basic recommendation filter (score, indie flag)
- [ ] Game list UI with search and filters

### Phase 2 — Wishlist and prices
- [ ] Steam wishlist importer
- [ ] IsThereAnyDeal price history integration
- [ ] Price history chart component
- [ ] Discount discovery view

### Phase 3 — Smart features
- [ ] Optimal buy time predictor
- [ ] Background sync daemon
- [ ] Price drop notifications
- [ ] Settings panel (Steam ID, thresholds, refresh intervals)

### Phase 4 — Polish
- [ ] Dark / light theme
- [ ] Performance profiling and optimisation
- [ ] CI/CD with GitHub Actions
- [ ] Packaged installer (Windows .msi, macOS .dmg)

---

## What I'm learning with this project

This is a portfolio project built to explore:

- **Rust** — ownership, async/await with Tokio, error handling with `anyhow`/`thiserror`, type-safe SQL with `sqlx`
- **Tauri** — Rust ↔ React IPC, desktop app distribution, native system integration
- **Performance architecture** — multi-layer caching, parallel async fetches, background sync patterns
- **API integration** — rate limiting, caching strategies, incremental sync
- **Local-first design** — app works offline after initial sync, no user data leaves the machine

---

## License

MIT
