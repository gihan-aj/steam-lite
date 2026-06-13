---
globs: "**/*.rs"
---

# Rust Rules for Steam Lite

## Error handling
- Use `anyhow::Result` for application-level errors
- Use `thiserror` for library-level errors with typed variants
- Never use `.unwrap()` in production code — use `?` or `.map_err()`
- Wrap all external API errors with context: `.map_err(|e| anyhow!("SteamSpy API: {}", e))`

## Async patterns
- Always use `tokio::spawn` for background tasks
- Use `tokio::join!` for parallel independent requests
- Use `Arc<Mutex<T>>` for shared state across async tasks
- Prefer `Arc<RwLock<T>>` when reads >> writes (e.g., caches)

## Performance
- Prefer `&str` over `String` in function parameters
- Use iterators and chains over explicit for-loops
- Avoid `.clone()` unless necessary — think about ownership first

## Documentation
- All `pub` functions get `///` doc comments
- Complex logic gets inline comments explaining *why*, not *what*