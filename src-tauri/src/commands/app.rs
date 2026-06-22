use serde::Serialize;

#[derive(Serialize)]
pub struct AppInfo {
    pub name: &'static str,
    pub version: &'static str,
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    AppInfo {
        name: "Nvy",
        version: env!("CARGO_PKG_VERSION"),
    }
}
