use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::assets::{AppDataPaths, CachedImage, ImageCacheKind};

#[tauri::command]
pub fn app_data_paths(app: AppHandle) -> Result<AppDataPaths, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    let database_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;

    crate::assets::app_data_paths(&app_data_dir, &database_path)
}

#[tauri::command]
pub fn cache_local_image(
    app: AppHandle,
    source_path: String,
    kind: String,
    owner_id: String,
) -> Result<CachedImage, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    let kind = ImageCacheKind::from_str(&kind)?;

    crate::assets::cache_local_image(&app_data_dir, &PathBuf::from(source_path), kind, &owner_id)
}

#[tauri::command]
pub fn resolve_cached_asset(app: AppHandle, relative_path: String) -> Result<String, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;

    crate::assets::resolve_cached_asset(&app_data_dir, &relative_path)
}
