use tauri::{AppHandle, State};
use tauri_plugin_opener::OpenerExt;
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

/// Returns the path to the log directory so the user can find logs.
#[tauri::command]
pub async fn get_log_path() -> Result<String, AppError> {
    let log_dir = dirs::data_dir()
        .ok_or_else(|| AppError::NotFound("Could not find app data directory".into()))?
        .join("steam-lite")
        .join("logs");

    Ok(log_dir.to_string_lossy().to_string())
}

/// Open the log directory in the system file explorer.
#[tauri::command]
pub async fn open_log_folder(app: AppHandle) -> Result<(), AppError> {
    let log_dir = dirs::data_dir()
        .ok_or_else(|| AppError::NotFound("Could not find app data directory".into()))?
        .join("steam-lite")
        .join("logs");

    // Use the opener plugin to open the folder
    app.opener().open_path(log_dir.to_string_lossy().as_ref(), None::<&str>)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to open folder: {}", e)))?;

    Ok(())
}