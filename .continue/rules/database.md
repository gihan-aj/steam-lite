---
globs: "src-tauri/src/db/**"
---

# Database Rules for Steam Lite

## SQLite / sqlx patterns
- Always use `query_as!` macro (compile-time checked)
- All queries go through Repository structs — no raw SQL in services
- Migrations in src-tauri/src/db/migrations/ with sequential numbering
- Always use parameterized queries — never string interpolation in SQL
- Price history is append-only — no UPDATE or DELETE on price_history table

## Schema conventions
- Primary keys: `id INTEGER PRIMARY KEY AUTOINCREMENT` or domain key (app_id)
- Timestamps: `created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`
- Booleans: INTEGER (0/1) — SQLite has no native bool
- JSON arrays: stored as TEXT, serialize/deserialize in repository layer