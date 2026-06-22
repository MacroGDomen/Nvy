use serde::Deserialize;
use tauri::AppHandle;

use crate::llm::{
    LlmSettings, LlmSettingsInput, RecommendationPayload, SecretStatus, TranslationState,
};

#[derive(Debug, Deserialize)]
pub struct LlmSettingsCommandInput {
    pub api_type: Option<String>,
    pub base_url: Option<String>,
    pub provider_name: Option<String>,
    pub model: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i64>,
    pub recommendation_reference_limit: i64,
    pub translation_prompt: Option<String>,
    pub recommendation_prompt: Option<String>,
    pub enable_llm_translation: bool,
    pub recommendation_default_enabled: bool,
    pub metadata_allow_browser_cookies: bool,
}

#[tauri::command]
pub fn get_llm_settings(app: AppHandle, account_id: String) -> Result<LlmSettings, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::llm::get_llm_settings(&db_path, &account_id)
}

#[tauri::command]
pub fn save_llm_settings(
    app: AppHandle,
    account_id: String,
    input: LlmSettingsCommandInput,
) -> Result<LlmSettings, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::llm::save_llm_settings(&db_path, &account_id, input.into())
}

#[tauri::command]
pub fn save_llm_api_key(account_id: String, api_key: String) -> Result<SecretStatus, String> {
    crate::llm::save_llm_api_key(&account_id, &api_key)
}

#[tauri::command]
pub fn clear_llm_api_key(account_id: String) -> SecretStatus {
    crate::llm::clear_llm_api_key(&account_id)
}

#[tauri::command]
pub fn build_recommendation_payload(
    app: AppHandle,
    account_id: String,
    user_text: String,
) -> Result<RecommendationPayload, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::llm::build_recommendation_payload(&db_path, &account_id, &user_text)
}

#[tauri::command]
pub fn request_video_translation(
    app: AppHandle,
    account_id: String,
    video_id: String,
) -> Result<TranslationState, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::llm::request_video_translation(&db_path, &account_id, &video_id)
}

#[tauri::command]
pub fn cancel_video_translation(
    app: AppHandle,
    account_id: String,
    video_id: String,
) -> Result<TranslationState, String> {
    let db_path = crate::db::database_path(&app).map_err(|error| error.to_string())?;
    crate::llm::cancel_video_translation(&db_path, &account_id, &video_id)
}

impl From<LlmSettingsCommandInput> for LlmSettingsInput {
    fn from(input: LlmSettingsCommandInput) -> Self {
        Self {
            api_type: input.api_type,
            base_url: input.base_url,
            provider_name: input.provider_name,
            model: input.model,
            temperature: input.temperature,
            max_tokens: input.max_tokens,
            recommendation_reference_limit: input.recommendation_reference_limit,
            translation_prompt: input.translation_prompt,
            recommendation_prompt: input.recommendation_prompt,
            enable_llm_translation: input.enable_llm_translation,
            recommendation_default_enabled: input.recommendation_default_enabled,
            metadata_allow_browser_cookies: input.metadata_allow_browser_cookies,
        }
    }
}
