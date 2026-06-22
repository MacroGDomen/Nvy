use serde::Deserialize;
use tauri::AppHandle;

use crate::tags::{TagInput, TagMatch, TagRecord};

#[derive(Debug, Deserialize)]
pub struct TagCommandInput {
    pub scope: String,
    pub canonical_name: String,
    pub aliases: Option<String>,
    pub related_tags: Option<String>,
}

#[tauri::command]
pub fn list_tags(
    app: AppHandle,
    account_id: String,
    scope: String,
) -> Result<Vec<TagRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::tags::list_tags(&db_path, &account_id, &scope)
}

#[tauri::command]
pub fn create_tag(
    app: AppHandle,
    account_id: String,
    input: TagCommandInput,
) -> Result<TagRecord, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::tags::create_tag(&db_path, &account_id, input.into())
}

#[tauri::command]
pub fn update_tag(
    app: AppHandle,
    account_id: String,
    tag_id: String,
    input: TagCommandInput,
) -> Result<TagRecord, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::tags::update_tag(&db_path, &account_id, &tag_id, input.into())
}

#[tauri::command]
pub fn delete_tag(app: AppHandle, account_id: String, tag_id: String) -> Result<(), String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::tags::delete_tag(&db_path, &account_id, &tag_id)
}

#[tauri::command]
pub fn match_tags(
    app: AppHandle,
    account_id: String,
    scope: String,
    query: String,
) -> Result<Vec<TagMatch>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::tags::match_tags(&db_path, &account_id, &scope, &query)
}

#[tauri::command]
pub fn list_video_tags(
    app: AppHandle,
    account_id: String,
    video_id: String,
) -> Result<Vec<TagRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::tags::list_video_tags(&db_path, &account_id, &video_id)
}

#[tauri::command]
pub fn set_video_tags(
    app: AppHandle,
    account_id: String,
    video_id: String,
    tag_ids: Vec<String>,
) -> Result<Vec<TagRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::tags::set_video_tags(&db_path, &account_id, &video_id, tag_ids)
}

#[tauri::command]
pub fn list_actress_tags(
    app: AppHandle,
    account_id: String,
    actress_id: String,
) -> Result<Vec<TagRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::tags::list_actress_tags(&db_path, &account_id, &actress_id)
}

#[tauri::command]
pub fn set_actress_tags(
    app: AppHandle,
    account_id: String,
    actress_id: String,
    tag_ids: Vec<String>,
) -> Result<Vec<TagRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::tags::set_actress_tags(&db_path, &account_id, &actress_id, tag_ids)
}

#[tauri::command]
pub fn auto_tag_video(
    app: AppHandle,
    account_id: String,
    video_id: String,
) -> Result<Vec<TagRecord>, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::tags::auto_tag_video(&db_path, &account_id, &video_id)
}

impl From<TagCommandInput> for TagInput {
    fn from(input: TagCommandInput) -> Self {
        Self {
            scope: input.scope,
            canonical_name: input.canonical_name,
            aliases: input.aliases,
            related_tags: input.related_tags,
        }
    }
}
