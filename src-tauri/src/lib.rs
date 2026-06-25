mod assets;
mod auth;
mod backup;
mod commands;
mod db;
mod defaults;
mod library;
mod llm;
mod metadata;
mod tags;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri::Manager;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("stronghold-salt.txt");
            app.handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::app_info,
            commands::assets::app_data_paths,
            commands::assets::cache_local_image,
            commands::assets::resolve_cached_asset,
            commands::auth::register_account,
            commands::auth::login_account,
            commands::auth::account_data_summary,
            commands::backup::export_plain_backup,
            commands::backup::export_encrypted_backup,
            commands::backup::import_plain_backup,
            commands::backup::import_encrypted_backup,
            commands::database::initialize_database,
            commands::defaults::initialize_account_defaults,
            commands::library::create_video,
            commands::library::update_video,
            commands::library::list_videos,
            commands::library::create_actress,
            commands::library::update_actress,
            commands::library::list_actresses,
            commands::library::list_video_actresses,
            commands::library::list_actress_videos,
            commands::library::set_video_actresses,
            commands::library::add_video_actress,
            commands::library::list_association_suggestions,
            commands::library::delete_video,
            commands::library::delete_actress,
            commands::metadata::match_video_metadata,
            commands::metadata::match_actress_metadata,
            commands::metadata::apply_metadata_candidate,
            commands::llm::get_llm_settings,
            commands::llm::save_llm_settings,
            commands::llm::save_llm_api_key,
            commands::llm::clear_llm_api_key,
            commands::llm::build_recommendation_payload,
            commands::llm::request_video_translation,
            commands::llm::request_llm_text,
            commands::llm::apply_video_translation,
            commands::llm::cancel_video_translation,
            commands::tags::list_tags,
            commands::tags::create_tag,
            commands::tags::update_tag,
            commands::tags::delete_tag,
            commands::tags::match_tags,
            commands::tags::list_video_tags,
            commands::tags::set_video_tags,
            commands::tags::list_actress_tags,
            commands::tags::set_actress_tags,
            commands::tags::auto_tag_video
        ])
        .run(tauri::generate_context!())
        .expect("error while running Nvy");
}
