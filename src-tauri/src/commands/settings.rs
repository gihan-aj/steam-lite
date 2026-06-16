use tauri::State;
use crate::AppState;
use crate::models::UserSettings;
use crate::error::AppError;

/// Load current settings from the database.
/// Called when the Settings page opens.
#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<UserSettings, AppError> {
  state.settings.load().await
}

/// Save a single setting value by key.
/// Called whenever the user changes a setting field.
///
/// 🦀 RUST LESSON: &str parameters in Tauri commands
/// Tauri deserializes the JS string directly into &str.
/// The 'static isn't needed here — Tauri handles the lifetime.
#[tauri::command]
pub async fn set_setting(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), AppError> {
    state.settings.set(&key, &value).await
}

/// Convenience command — save the whole settings object at once.
#[tauri::command]
pub async fn save_settings(
    state: State<'_, AppState>,
    settings: UserSettings,
) -> Result<(), AppError> {
    state.settings.save(&settings).await
}