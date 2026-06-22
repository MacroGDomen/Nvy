use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::backup::{BackupResult, ImportResult};

#[tauri::command]
pub fn export_plain_backup(
    app: AppHandle,
    account_id: String,
    output_path: String,
) -> Result<BackupResult, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    crate::backup::export_plain_backup(
        &db_path,
        &app_data_dir,
        &account_id,
        &PathBuf::from(output_path),
    )
}

#[tauri::command]
pub fn export_encrypted_backup(
    app: AppHandle,
    account_id: String,
    output_path: String,
    password: String,
) -> Result<BackupResult, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    crate::backup::export_encrypted_backup(
        &db_path,
        &app_data_dir,
        &account_id,
        &PathBuf::from(output_path),
        &password,
    )
}

#[tauri::command]
pub fn import_plain_backup(
    app: AppHandle,
    account_id: String,
    input_path: String,
) -> Result<ImportResult, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    crate::backup::import_plain_backup(
        &db_path,
        &app_data_dir,
        &account_id,
        &PathBuf::from(input_path),
    )
}

#[tauri::command]
pub fn import_encrypted_backup(
    app: AppHandle,
    account_id: String,
    input_path: String,
    password: String,
) -> Result<ImportResult, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    crate::backup::import_encrypted_backup(
        &db_path,
        &app_data_dir,
        &account_id,
        &PathBuf::from(input_path),
        &password,
    )
}
