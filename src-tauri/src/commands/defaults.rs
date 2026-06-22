use tauri::AppHandle;

use crate::defaults::AccountDefaults;

#[tauri::command]
pub fn initialize_account_defaults(
    app: AppHandle,
    account_id: String,
) -> Result<AccountDefaults, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::defaults::initialize_account_defaults(&db_path, &account_id)
}
