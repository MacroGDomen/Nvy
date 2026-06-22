use serde::Deserialize;
use tauri::AppHandle;

use crate::library::{ActressInput, ActressRecord, AssociationSuggestion, VideoInput, VideoRecord};

#[derive(Debug, Deserialize)]
pub struct VideoCommandInput {
    pub code: String,
    pub title: Option<String>,
    pub cover_path: Option<String>,
    pub release_date: Option<String>,
    pub duration_minutes: Option<i64>,
    pub source_url: Option<String>,
    pub summary: Option<String>,
    pub actor_names: Option<String>,
    pub work_type: String,
    pub review: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActressCommandInput {
    pub name: String,
    pub simplified_chinese_name: Option<String>,
    pub former_chinese_names: Option<String>,
    pub traditional_chinese_name: Option<String>,
    pub japanese_name: Option<String>,
    pub romanized_name: Option<String>,
    pub default_display_name_type: Option<String>,
    pub avatar_path: Option<String>,
    pub measurements: Option<String>,
    pub cup_size: Option<String>,
    pub birthday: Option<String>,
    pub height_cm: Option<i64>,
    pub debut_date: Option<String>,
    pub wikipedia_zh_url: Option<String>,
    pub note: Option<String>,
}

#[tauri::command]
pub fn create_video(
    app: AppHandle,
    account_id: String,
    input: VideoCommandInput,
) -> Result<VideoRecord, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::create_video(&db_path, &account_id, input.into())
}

#[tauri::command]
pub fn update_video(
    app: AppHandle,
    account_id: String,
    video_id: String,
    input: VideoCommandInput,
) -> Result<VideoRecord, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::update_video(&db_path, &account_id, &video_id, input.into())
}

#[tauri::command]
pub fn list_videos(app: AppHandle, account_id: String) -> Result<Vec<VideoRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::list_videos(&db_path, &account_id)
}

#[tauri::command]
pub fn create_actress(
    app: AppHandle,
    account_id: String,
    input: ActressCommandInput,
) -> Result<ActressRecord, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::create_actress(&db_path, &account_id, input.into())
}

#[tauri::command]
pub fn update_actress(
    app: AppHandle,
    account_id: String,
    actress_id: String,
    input: ActressCommandInput,
) -> Result<ActressRecord, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::update_actress(&db_path, &account_id, &actress_id, input.into())
}

#[tauri::command]
pub fn list_actresses(app: AppHandle, account_id: String) -> Result<Vec<ActressRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::list_actresses(&db_path, &account_id)
}

#[tauri::command]
pub fn list_video_actresses(
    app: AppHandle,
    account_id: String,
    video_id: String,
) -> Result<Vec<ActressRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::list_video_actresses(&db_path, &account_id, &video_id)
}

#[tauri::command]
pub fn list_actress_videos(
    app: AppHandle,
    account_id: String,
    actress_id: String,
) -> Result<Vec<VideoRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::list_actress_videos(&db_path, &account_id, &actress_id)
}

#[tauri::command]
pub fn set_video_actresses(
    app: AppHandle,
    account_id: String,
    video_id: String,
    actress_ids: Vec<String>,
) -> Result<Vec<ActressRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::set_video_actresses(&db_path, &account_id, &video_id, actress_ids)
}

#[tauri::command]
pub fn add_video_actress(
    app: AppHandle,
    account_id: String,
    video_id: String,
    actress_id: String,
) -> Result<Vec<ActressRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::add_video_actress(&db_path, &account_id, &video_id, &actress_id)
}

#[tauri::command]
pub fn list_association_suggestions(
    app: AppHandle,
    account_id: String,
    actress_id: String,
) -> Result<Vec<AssociationSuggestion>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::list_association_suggestions(&db_path, &account_id, &actress_id)
}

#[tauri::command]
pub fn delete_video(app: AppHandle, account_id: String, video_id: String) -> Result<(), String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::delete_video(&db_path, &account_id, &video_id)
}

#[tauri::command]
pub fn delete_actress(
    app: AppHandle,
    account_id: String,
    actress_id: String,
) -> Result<(), String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::library::delete_actress(&db_path, &account_id, &actress_id)
}

impl From<VideoCommandInput> for VideoInput {
    fn from(input: VideoCommandInput) -> Self {
        Self {
            code: input.code,
            title: input.title,
            cover_path: input.cover_path,
            release_date: input.release_date,
            duration_minutes: input.duration_minutes,
            source_url: input.source_url,
            summary: input.summary,
            actor_names: input.actor_names,
            work_type: input.work_type,
            review: input.review,
        }
    }
}

impl From<ActressCommandInput> for ActressInput {
    fn from(input: ActressCommandInput) -> Self {
        Self {
            name: input.name,
            simplified_chinese_name: input.simplified_chinese_name,
            former_chinese_names: input.former_chinese_names,
            traditional_chinese_name: input.traditional_chinese_name,
            japanese_name: input.japanese_name,
            romanized_name: input.romanized_name,
            default_display_name_type: input.default_display_name_type,
            avatar_path: input.avatar_path,
            measurements: input.measurements,
            cup_size: input.cup_size,
            birthday: input.birthday,
            height_cm: input.height_cm,
            debut_date: input.debut_date,
            wikipedia_zh_url: input.wikipedia_zh_url,
            note: input.note,
        }
    }
}
