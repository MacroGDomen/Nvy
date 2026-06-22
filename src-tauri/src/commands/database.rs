use serde::Serialize;
use tauri::AppHandle;

#[derive(Serialize)]
pub struct DatabaseStatus {
    pub path: String,
    pub schema_version: i64,
}

#[tauri::command]
pub fn initialize_database(app: AppHandle) -> Result<DatabaseStatus, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    let schema_version =
        crate::db::initialize_database(&db_path).map_err(|error| error.to_string())?;

    Ok(DatabaseStatus {
        path: db_path.display().to_string(),
        schema_version,
    })
}
