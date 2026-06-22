mod auth;
mod commands;
mod db;
mod defaults;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::app::app_info,
            commands::auth::register_account,
            commands::auth::login_account,
            commands::auth::account_data_summary,
            commands::database::initialize_database,
            commands::defaults::initialize_account_defaults
        ])
        .run(tauri::generate_context!())
        .expect("error while running Nvy");
}
