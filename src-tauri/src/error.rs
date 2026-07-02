use thiserror::Error;

/// The main error type for Steam Lite.
#[derive(Debug, Error)]
pub enum AppError {
    /// Returned when an external API call fails.
    #[error("API Error: {0}")]
    Api(String),

    /// Returned when a database operation fails.
    #[error("Database Error: {0}")]
    Database(#[from] sqlx::Error),

    /// Returned when we can't parse/deserialize API response JSON.
    #[error("Failed to parse response: {0}")]
    Parse(String),

    /// Returned when a requested resource doesn't exist.
    #[error("Not found: {0}")]
    NotFound(String),

    /// A catch-all for unexpected errors.
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    /// User-facing message - safe to show in the UI
    pub fn user_message(&self) -> &str {
        match self {
            AppError::Api(msg) => {
                // Classify common API errors into friendly messages
                if msg.contains("403") || msg.contains("401") {
                    "API key is invalid or expired. Check your settings."
                } else if msg.contains("404") {
                    "Content not found. It may have been removed."
                } else if msg.contains("429") {
                    "Too many requests. Please wait a moment and try again."
                } else if msg.contains("timed out") || msg.contains("timeout") {
                    "Request timed out. Check your internet connection."
                } else if msg.contains("connection") || msg.contains("dns") {
                    "Could not connect. Check your internet connection."
                } else {
                    "An external service returned an error. Try again later."
                }
            }
            AppError::Database(_) =>
                "A database error occurred. Try restarting the app.",
            AppError::Parse(_) =>
                "Received unexpected data from an external service.",
            AppError::NotFound(msg) =>
                msg.as_str(),   // NotFound messages are already user-friendly
            AppError::Internal(_) =>
                "An unexpected error occurred. Please check the logs.",
        }
    } 

    /// Whether this error is worth showing to the user at all
    pub fn is_user_visible(&self) -> bool {
        match self {
            AppError::NotFound(_) => true,
            AppError::Api(_)      => true,
            AppError::Database(_) => true,
            AppError::Parse(_)    => false,  // technical, user can't act on it
            AppError::Internal(_) => true,
        }
    }

    /// Log level for this error
    pub fn log_level(&self) -> &str {
        match self {
            AppError::NotFound(_) => "warn",
            AppError::Api(_)      => "error",
            AppError::Database(_) => "error",
            AppError::Parse(_)    => "error",
            AppError::Internal(_) => "error",
        }
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: serde::Serializer {
        // Send a structured error to React, not just a string
        // React can use `type` for classification and `message` for display
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("type", &format!("{:?}", self)
            .split('(').next().unwrap_or("Unknown"))?;
        map.serialize_entry("message", &self.user_message())?;
        map.serialize_entry("detail", &self.to_string())?;
        map.end()
    }
}

/// A convenience alias so we can write `Result<T>` instead of `Result<T, AppError>`
pub type Result<T> = std::result::Result<T, AppError>;