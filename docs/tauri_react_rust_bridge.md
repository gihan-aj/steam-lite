# Tauri, Commands, Events, and TanStack Query

This document explains how the React frontend and Rust backend talk to each other
in this app, and what TanStack Query is doing on top of that. All examples are from
real code in this project.

---

## 1. What is Tauri?

Tauri is a framework for building desktop apps. Instead of shipping a heavy
Electron runtime (which bundles a full Chromium browser + Node.js), Tauri uses:

- The **OS's own WebView** to render your React UI (Edge on Windows, WebKit on macOS/Linux).
- A **Rust binary** as the backend/process that has full access to the OS, filesystem, network, etc.

The result is a very small, fast native app where React draws the UI and Rust does all the
heavy lifting — API calls, database reads/writes, background tasks.

```
┌──────────────────────────────────────────────┐
│  Your Window (WebView)                        │
│  React + TypeScript + Tailwind                │
│                                               │
│  invoke("get_crawl_state") ─────────────────► │ ──► Rust backend
│  ◄──────────── { status: "paused", ... }     │ ◄── returns JSON
└──────────────────────────────────────────────┘
```

The bridge between the two sides is the **Tauri IPC channel** (Inter-Process Communication).
It is what `invoke()` and `listen()` use under the hood.

---

## 2. Tauri Commands — React calling Rust

A **command** is the primary way React asks Rust to do something and get a result back.
Think of it like a function call that crosses the process boundary.

### Rust side — defining a command

A command is any Rust `async fn` decorated with `#[tauri::command]`:

```rust
// src-tauri/src/commands/discover.rs

#[tauri::command]
pub async fn get_crawl_state(
    state: State<'_, AppState>,
) -> Result<CrawlState, AppError> {
    state.crawl.load().await   // reads from SQLite, returns a struct
}
```

Key points:
- `State<'_, AppState>` is Tauri's way of injecting shared app state (DB pool, API clients, etc.)
  You don't pass this from React — Tauri fills it in automatically.
- The return type must implement `serde::Serialize` so Tauri can convert it to JSON for React.
- `AppError` implements `serde::Serialize` too — Tauri turns errors into rejected JS Promises.

The command must be **registered** in `lib.rs` inside `invoke_handler`:

```rust
// src-tauri/src/lib.rs

.invoke_handler(tauri::generate_handler![
    commands::discover::get_crawl_state,
    commands::discover::start_crawl,
    commands::discover::stop_crawl,
    commands::discover::reset_crawl,
    commands::discover::get_hidden_gems,
])
```

`generate_handler![]` is a macro that builds the routing table. If a command is missing from
this list, React calling it will get an error — the macro call is the "registration" step.

### React side — calling a command

```typescript
// src/hooks/useDiscover.ts

import { invoke } from "@tauri-apps/api/core";

const crawlState = await invoke<CrawlState>("get_crawl_state");
```

`invoke<T>(commandName, args?)` is an `async` function that:
1. Serializes the args object to JSON
2. Sends it across the IPC channel to Rust
3. Waits for Rust to finish
4. Deserializes the returned JSON back into TypeScript type `T`
5. Throws if Rust returned an `Err`

The `<T>` generic is just a TypeScript cast — Tauri doesn't validate it at runtime.
You are responsible for keeping the Rust struct and the TypeScript interface in sync.

### Passing arguments

Arguments are passed as a plain object, matching Rust parameter names **exactly** (snake_case):

```typescript
// React
invoke<DiscoverGame[]>("get_hidden_gems", { limit: 100 });
```

```rust
// Rust
#[tauri::command]
pub async fn get_hidden_gems(
    state: State<'_, AppState>,
    limit: i64,    // ← "limit" must match exactly
) -> Result<Vec<Game>, AppError> {
    state.games.get_hidden_gems(limit).await
}
```

---

## 3. AppState — the shared Rust context

`AppState` is a struct registered once at startup that holds all the things the backend needs:

```rust
// src-tauri/src/lib.rs

pub struct AppState {
    pub db:            SqlitePool,
    pub games:         GameRepository,
    pub crawl:         CrawlRepository,
    pub steamspy:      SteamSpyClient,
    pub crawl_stop_tx: watch::Sender<bool>,   // the "stop the crawl" signal
    pub crawl_task:    Mutex<Option<JoinHandle<()>>>,
    // ...
}
```

At startup in `setup()`:
```rust
let state = AppState { ... };
app.manage(state);   // Tauri now owns it and injects it into every command
```

Every command that declares `State<'_, AppState>` as a parameter gets a reference to this
same object. It lives for the entire lifetime of the app. This is how all commands share
the same database pool, API clients, and channel senders without any global variables.

---

## 4. Tauri Events — Rust pushing data to React

Commands are **request-response**: React asks, Rust answers, done.

Events are **push notifications**: Rust decides to send something to React at any time,
React just listens. This is essential for background tasks like the crawl — you can't
have React polling every 100ms waiting for a page to finish.

### Rust side — emitting an event

```rust
// src-tauri/src/services/catalogue_crawler.rs

use tauri::{AppHandle, Emitter};

fn emit_progress(app: &AppHandle, crawl: &CrawlState, ...) {
    let _ = app.emit("crawl_progress", CrawlProgress {
        current_page:    14,
        total_pages:     50,
        games_indexed:   13000,
        games_qualified: 6672,
        status:          "running".to_string(),
        percent:         28,
        wait_seconds:    None,
    });
}
```

`app.emit(event_name, payload)` broadcasts to ALL windows. The payload is serialized to JSON.
`AppHandle` is how the background Tokio task has access to the Tauri runtime — it's cloned
from the main `App` handle and passed into the spawned task when it starts.

### React side — listening for an event

```typescript
// src/hooks/useDiscover.ts

import { listen } from "@tauri-apps/api/event";

useEffect(() => {
    const unlisten = listen<CrawlProgress>("crawl_progress", (event) => {
        // event.payload is already deserialized to CrawlProgress
        setLiveProgress(event.payload);
    });

    // Cleanup: unsubscribe when component unmounts
    return () => { unlisten.then(fn => fn()); };
}, []);
```

`listen<T>(event_name, handler)` returns a Promise that resolves to an "unlisten" function.
You **must** call that unlisten function when the component unmounts (in the `useEffect`
cleanup) — otherwise you accumulate listeners across re-renders and get memory leaks.

### Events vs Commands — when to use which

| Use a **Command** when... | Use an **Event** when... |
|---|---|
| React needs a result right now | Rust needs to push data to React unprompted |
| Loading data for a page | Background task reporting progress |
| User clicks a button, expects a response | Sync finishing, crawl page done, price alert |
| The operation is short (< a few seconds) | Long-running operation that React should just watch |

In this app:
- `invoke("get_crawl_state")` → command (React requests current state from DB)
- `"crawl_progress"` event → pushed by the crawl task after each page
- `invoke("start_crawl")` → command (returns immediately, spawns background task)
- `"prices_refreshed"` event → pushed by the background sync after it completes
- `"sync_due"` event → pushed at startup to tell React to trigger a sync

---

## 5. The Full Crawl Flow — Putting It Together

This is a concrete walkthrough of what happens when you click **▶ Resume**:

```
React (Discover.tsx)
│  onClick={() => start.mutate()}
│  calls useStartCrawl hook's mutationFn
│
│  invoke("start_crawl")
│  ──────────────────────────────────────────────────────────►
│                                                     Rust (commands/discover.rs)
│                                                     start_crawl():
│                                                       1. abort old task handle
│                                                       2. crawl_stop_tx.send_replace(false)
│                                                       3. crawl.set_status(Running) → DB
│                                                       4. stop_rx = subscribe()
│                                                       5. spawn background Tokio task
│                                                       6. store JoinHandle
│                                                       7. return Ok(())
│  ◄──────────────────────────────────────────────────────────
│  onSuccess():
│    setQueryData(["crawl_state"], { ...old, status: "running" })  ← optimistic update
│    setTimeout(invalidateQueries, 800ms)                          ← real DB refetch later
│
│  [Background Tokio task is now running independently]
│
│                                                     Rust (catalogue_crawler.rs)
│                                                     run_crawl_with_handle():
│                                                       check last_page_at → need to wait 50s
│                                                       emit("crawl_progress", { status: "waiting", wait_seconds: 50 })
│  ◄──────────────────────────────────────────────────────────
│  useCrawlProgress listener fires:
│    setLiveProgress({ status: "waiting", wait_seconds: 50 })
│    start setInterval countdown
│    UI shows "Rate limit — resuming in 50s"
│
│  [50 seconds pass...]
│
│                                                     sleep arm completes
│                                                     fetch page 14 from SteamSpy
│                                                     score games, upsert to DB
│                                                     advance_page(14, ...) → DB
│                                                     emit("crawl_progress", { status: "running", ... })
│  ◄──────────────────────────────────────────────────────────
│  listener fires:
│    setLiveProgress({ status: "running", current_page: 15, ... })
│    clearInterval countdown
│    UI shows progress bar advancing
│
│  [62 second sleep starts for next page...]
```

---

## 6. Data Types Must Match on Both Sides

Tauri serializes Rust structs to JSON with `serde`. The TypeScript interfaces in
`src/types/index.ts` must mirror the Rust structs exactly.

**Rust struct:**
```rust
// src-tauri/src/services/catalogue_crawler.rs

#[derive(serde::Serialize, Clone)]
pub struct CrawlProgress {
    pub current_page:    u32,
    pub total_pages:     u32,
    pub games_indexed:   u32,
    pub games_qualified: u32,
    pub status:          String,
    pub percent:         u8,
    pub wait_seconds:    Option<u32>,  // Option<T> → T | null in JSON
}
```

**TypeScript interface:**
```typescript
// src/types/index.ts

export interface CrawlProgress {
  current_page:    number;
  total_pages:     number;
  games_indexed:   number;
  games_qualified: number;
  status:          string;
  percent:         number;
  wait_seconds?:   number;   // optional = may be absent = was Option<u32> on Rust side
}
```

Field naming rules:
- Rust uses `snake_case` for struct fields by default — so does the JSON.
- TypeScript interfaces should also use `snake_case` to match.
- `Option<T>` in Rust → the field is either a value or `null` in JSON → `T | null` or `T?` in TypeScript.
- If Rust uses `#[serde(rename_all = "camelCase")]`, TypeScript must use `camelCase` too.
  In this project, no such rename is used — everything stays `snake_case`.

---

## 7. TanStack Query — Managing Server State in React

### The Problem It Solves

Without TanStack Query you'd write React like this for every piece of data:

```typescript
const [games, setGames]     = useState(null);
const [loading, setLoading] = useState(true);
const [error, setError]     = useState(null);

useEffect(() => {
    setLoading(true);
    invoke("get_hidden_gems", { limit: 100 })
        .then(setGames)
        .catch(setError)
        .finally(() => setLoading(false));
}, []);
```

For a multi-page app with shared data (crawl state visible in multiple components, wishlist,
prices...) this becomes a nightmare: duplicate requests, stale data, no cache, every component
refetches on mount.

**TanStack Query** (formerly React Query) is a library that manages this "server state" — data
that lives on the backend and needs to be fetched, cached, and kept in sync. It gives you:

- A **cache** keyed by query keys
- **Deduplication** (two components calling the same query = one network request)
- **Automatic background refetching** (stale-while-revalidate)
- **Loading/error states** built in
- **Mutations** (write operations) with `onSuccess` callbacks

### `useQuery` — Reading Data

```typescript
// src/hooks/useDiscover.ts

export function useCrawlState() {
    return useQuery({
        queryKey: ["crawl_state"],   // ← cache key
        queryFn:  () => invoke<CrawlState>("get_crawl_state"),
        refetchInterval: 5000,       // poll every 5 seconds as a fallback
        staleTime: 0,                // always consider data stale (always refetch on mount)
    });
}
```

```typescript
// src/pages/Discover.tsx

const { data: crawlState, isLoading: crawlLoading } = useCrawlState();
```

What TanStack Query does behind the scenes:
1. First render: `data` is `undefined`, `isLoading` is `true` → calls `queryFn`
2. `queryFn` resolves: `data` = `CrawlState`, `isLoading` = `false` → component re-renders
3. Result is cached under key `["crawl_state"]`
4. Another component calling `useCrawlState()` → hits cache, NO second `invoke()` call
5. Every 5 seconds: re-runs `queryFn`, updates cache, re-renders if data changed

### `useMutation` — Writing Data / Triggering Actions

```typescript
// src/hooks/useDiscover.ts

export function useStartCrawl() {
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: () => invoke<void>("start_crawl"),

        onSuccess: () => {
            // Optimistic update: immediately tell the cache the status is "running"
            // without waiting for a DB round-trip
            queryClient.setQueryData(["crawl_state"], (old: CrawlState | undefined) =>
                old ? { ...old, status: "running" } : old
            );

            // Real refetch 800ms later to get the authoritative DB value
            setTimeout(() => {
                queryClient.invalidateQueries({ queryKey: ["crawl_state"] });
            }, 800);
        },
    });
}
```

```typescript
// src/pages/Discover.tsx

const start = useStartCrawl();

// ...
<button onClick={() => start.mutate()}>▶ Resume</button>

// start.isPending is true while the invoke() is in flight
// start.isError / start.error if Rust returned Err
```

Key `useMutation` concepts:
- `mutationFn`: the async function to run (the `invoke`)
- `isPending`: `true` while waiting for Rust to respond (used to disable the button)
- `onSuccess`: fires after the Rust command returns `Ok` — used to update the cache
- `onError`: fires if Rust returned an `Err`

### `invalidateQueries` vs `setQueryData`

| Method | What it does |
|---|---|
| `invalidateQueries({ queryKey })` | Marks the cache entry as stale → triggers a background refetch → components re-render with new data |
| `setQueryData(queryKey, updater)` | Directly writes a new value into the cache → components re-render immediately, **no network call** |

In the crawl feature, both are used together:
1. `setQueryData` to instantly show "running" (optimistic update)
2. `invalidateQueries` 800ms later to confirm with the real DB value

This pattern prevents the UI from feeling sluggish when the backend command returns quickly
but the DB hasn't been updated yet.

### The QueryClient and QueryClientProvider

The cache lives in a `QueryClient`. It's created once at the root of the app and provided
to all components via `QueryClientProvider`:

```typescript
// src/main.tsx (typical setup)

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root")!).render(
    <QueryClientProvider client={queryClient}>
        <App />
    </QueryClientProvider>
);
```

Any component can access the cache via `useQueryClient()`.

---

## 8. How Events and TanStack Query Work Together

Events (Rust pushing) and TanStack Query (managing cache) are complementary:

```typescript
// src/hooks/useDiscover.ts

export function useCrawlProgress() {
    const queryClient = useQueryClient();
    const [liveProgress, setLiveProgress] = useState<CrawlProgress | null>(null);

    useEffect(() => {
        const unlisten = listen<CrawlProgress>("crawl_progress", (event) => {
            const p = event.payload;

            // 1. Store the live event payload in local state for instant UI updates
            setLiveProgress(p);

            // 2. Also tell TanStack Query the crawl_state cache is stale
            //    so the next useCrawlState() call gets fresh DB data
            queryClient.invalidateQueries({ queryKey: ["crawl_state"] });

            // 3. When crawl finishes, also refresh the gems list
            if (p.status === "complete" || p.status === "paused") {
                queryClient.invalidateQueries({ queryKey: ["hidden_gems"] });
            }
        });

        return () => { unlisten.then(fn => fn()); };
    }, [queryClient]);

    return { liveProgress, countdown };
}
```

The pattern:
- `liveProgress` (local state) = real-time, updated every time the Rust task emits an event.
  Used for the progress bar and countdown during active crawling.
- `crawlState` from `useCrawlState()` (TanStack cache) = the authoritative DB snapshot.
  Used to correctly resume after navigating away and back, or after restart.

In `Discover.tsx` they are combined:

```typescript
// Use liveProgress if it's current, fall back to DB state
const liveIsStale =
    liveProgress !== null &&
    crawlState !== undefined &&
    (liveProgress.status === "paused" || liveProgress.status === "complete") &&
    crawlState.status === "running";

const progress = liveIsStale || liveProgress === null
    ? crawlState-derived-object   // DB wins when live state is stale
    : liveProgress;               // Live event wins during active crawl
```

This is why navigating away and back to Discover still shows correct progress:
`liveProgress` resets to `null` (component unmounts, listener removed), but
`useCrawlState()` still has the cached DB value from the last refetch.

---

## 9. The watch Channel — Rust-to-Rust Signalling

This isn't React/Tauri-specific, but it's important for understanding how `stop_crawl` works.

`tokio::sync::watch` is a channel that holds **one value** and notifies receivers whenever it changes.
Unlike `mpsc` (multi-producer, single-consumer) or `oneshot`, watch is designed for
"broadcast the latest state" scenarios.

```rust
// lib.rs — created once at startup
let (crawl_stop_tx, _) = watch::channel(false);
// crawl_stop_tx lives in AppState
```

```rust
// stop_crawl command
state.crawl_stop_tx.send_replace(true);   // signal: "please stop"
```

```rust
// Inside run_crawl_with_handle (the background task)
// "stop_rx" is a Receiver connected to the same channel

// At the top of every loop iteration:
if *stop_rx.borrow() {
    // signal is true → save state, exit cleanly
    return Ok(());
}

// During the 62-second sleep between pages:
tokio::select! {
    _ = sleep(62s) => { /* normal: proceed to next page */ }
    _ = stop_rx.changed() => {
        if *stop_rx.borrow_and_update() {
            return Ok(());  // stop signal received during sleep
        }
        // else: value changed to false (resume signal), continue
    }
}
```

`tokio::select!` runs multiple async operations concurrently and returns when the FIRST one
completes. This is how the 62-second sleep can be interrupted instantly when the user clicks
Pause — instead of waiting the full minute.

`send_replace()` is critical because `send()` silently fails when there are no active receivers.
Since the task drops its receiver when it exits, the next call to `send()` (resetting to `false`
on resume) would do nothing, leaving the channel stuck at `true` and causing the newly started
task to immediately stop. `send_replace()` always writes the value regardless.

---

## 10. Quick Reference

### Rust → React (Event)
```rust
app.emit("event_name", payload_struct);   // payload must derive Serialize
```
```typescript
const unlisten = listen<PayloadType>("event_name", (e) => { ... });
// cleanup: unlisten.then(fn => fn())
```

### React → Rust (Command)
```rust
#[tauri::command]
pub async fn my_command(state: State<'_, AppState>, arg: String) -> Result<MyReturn, AppError> { }
// register in generate_handler![]
```
```typescript
const result = await invoke<MyReturn>("my_command", { arg: "value" });
```

### TanStack Query
```typescript
// Read
const { data, isLoading, error } = useQuery({ queryKey: ["key"], queryFn: () => invoke(...) });

// Write
const mutation = useMutation({ mutationFn: () => invoke(...), onSuccess: () => queryClient.invalidateQueries(...) });
mutation.mutate();  // trigger it
mutation.isPending  // true while in flight

// Manual cache ops
queryClient.setQueryData(["key"], newValue);               // write directly
queryClient.invalidateQueries({ queryKey: ["key"] });      // mark stale → refetch
```
