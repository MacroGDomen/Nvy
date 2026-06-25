use tauri::{AppHandle, Manager};

use crate::metadata::MetadataCandidate;

#[tauri::command]
pub async fn match_video_metadata(
    app: AppHandle,
    account_id: String,
    video_id: String,
    query: String,
) -> Result<Vec<MetadataCandidate>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    tauri::async_runtime::spawn_blocking(move || {
        crate::metadata::match_video_metadata(&db_path, &account_id, &video_id, &query)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn match_actress_metadata(
    app: AppHandle,
    account_id: String,
    actress_id: String,
    query: String,
) -> Result<Vec<MetadataCandidate>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    tauri::async_runtime::spawn_blocking(move || {
        crate::metadata::match_actress_metadata(
            &db_path,
            &app_data_dir,
            &account_id,
            &actress_id,
            &query,
        )
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn apply_metadata_candidate(
    app: AppHandle,
    account_id: String,
    candidate_id: String,
) -> Result<MetadataCandidate, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    tauri::async_runtime::spawn_blocking(move || {
        crate::metadata::apply_metadata_candidate(
            &db_path,
            &app_data_dir,
            &account_id,
            &candidate_id,
        )
    })
    .await
    .map_err(|error| error.to_string())?
}
