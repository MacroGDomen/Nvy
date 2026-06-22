use tauri::AppHandle;

use crate::auth::{AccountDataSummary, AccountSession};

#[tauri::command]
pub fn register_account(
    app: AppHandle,
    username: String,
    password: String,
) -> Result<AccountSession, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::auth::register_account(&db_path, &username, &password)
}

#[tauri::command]
pub fn login_account(
    app: AppHandle,
    username: String,
    password: String,
) -> Result<AccountSession, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::auth::login_account(&db_path, &username, &password)
}

#[tauri::command]
pub fn account_data_summary(
    app: AppHandle,
    account_id: String,
) -> Result<AccountDataSummary, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::auth::account_data_summary(&db_path, &account_id)
}
