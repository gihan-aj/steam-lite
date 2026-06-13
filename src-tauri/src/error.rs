use thiserror::Error;

/// The main error type for Steam Lite.
///
/// `#[derive(Error)]` is a macro from the `thiserror` crate.
/// It automatically implements the standard `Error` trait for us.
///
/// Each variant is like a specific Exception subclass in C#:
///   AppError::Api      → like ApiException
///   AppError::Database → like DatabaseException
///   etc.
#[derive(Debug, Error)]
pub enum AppError {
    /// Returned when an external API call fails.
    /// The `{0}` in the message refers to the first field of the variant.
    #[error("API Error: {0}")]
    Api(String),

    /// Returned when a database operation fails.
    /// `#[from]` means sqlx::Error automatically converts INTO AppError::Database.
    /// This is what makes the `?` operator work on database calls.
    #[error("Database Error: {0}")]
    Database(#[from] sqlx::Error),

    /// Returned when we can't parse/deserialize API response JSON.
    #[error("Failed to parse response: {0}")]
    Parse(String),

    /// Returned when a requested resource doesn't exist.
    #[error("Not found: {0}")]
    NotFound(String),

    /// A catch-all for unexpected errors.
    /// `#[from]` lets any anyhow::Error convert into this variant.
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

/// A convenience alias so we can write `Result<T>` instead of `Result<T, AppError>`
pub type Result<T> = std::result::Result<T, AppError>;

// Tauri commands return results to the frontend (React). For that to work,
// the error type must implement `serde::Serialize` so it can become JSON.
// We implement it manually here to give the frontend a consistent error shape:
//   { "error": "API error: connection timed out" }
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Convert the error to its display string (the #[error("...")] message)
        // and serialize that. Simple and readable on the frontend.
        serializer.serialize_str(&self.to_string())
    }
}